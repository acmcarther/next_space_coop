# Unify Sockets, Timers, and Channels

Currently, there are two runtime APIs: `Poll` and `EventLoop`. `Poll` is
the abstraction handling readiness for sockets and waiting for events on
these sockets. `EventLoop` is a wrapper around `Poll` providing a
timeout API and a cross thread notification API as well as the loop /
reactive (via `Handler`) abstraction.

I am proposing to extract the timer and the notification channel
features into standalone types that implement `Evented` and thus can be
used directly with `Poll`. For example:

```rust
let timer = mio::Timer::new();
let poll = mio::Poll::new();

poll.register(&timer, Token(0), EventSet::readable(), PollOpt::edge());

timer.timeout("hello", Duration::from_millis(1_000));

poll.poll();
```

The insight is that a lot of the code that is currently windows specific
is useful to all platforms. Stabilizing the API and providing it to all
platforms allows implementing `Evented` for arbitrary types.

## Advantages

- Mio would have a unified API.
- No change in overhead for any platform.
- Move (and improve) some of the currently windows-only code to all platforms.
- The notification channel backend could be configurable:

```rust
let (tx, rx) = mio::Channel::new(mpsc::channel());
poll.register(&rx, Token(0), EventSet::readable(), PollOpt::edge());
```

## Disadvantages

* Unsafe code
* More code (lock-free algorithms)

The primary disadvantage that I can think of is that the code path
around timers & the notification channel become slightly more
complicated. I don't believe that the change would have a meaningful
performance impact.

There is also additional code complexity for all platforms. However,
this code complexity already exists for Windows.

## Behavior

An `Evented` would mirror the behavior of a socket registered with
epoll. Specifically, in a single threaded environment:

* A value registered will trigger at most one notification per call to `poll`.
* A value registered with readable interest & edge triggers a notification once when it becomes readable.
* A value registered with readable interest & level triggers a notification every call to poll as long as the value is still readable.
* A value registered (or reregistered) with readable interest triggers a notification immediately if it is currently readable.
* If a value is registered with readable interest only and already has a pending writable notification, the event is discarded
* If a value has any pending notifications and is deregistered, the pending notifications are cleared.
* When a value is dropped, it will no longer trigger any further notifications.
* `Poll` is permitted to fire of spurious readiness events *except* if the value has been dropped.

In the presence of concurrency, specifically readiness being modified on
a different thread than `Poll`, a best effort is made to preserve these
semantics.

## Implementation

This section will describe how to implement a custom `Evented` type as
well as Mio's internals to handle it. For simplicity and performance,
custom `Evented` types will only be able to be registered with a single
`Poll`.

It is important to note that the implementation is **not** intended to
replace FD polling on epoll & kqueue. It is meant to work in conjunction
with the OS's event queue to support types that cannot be implemented
using a socket or other system type that is compatible with the system's
event queue.

### Readiness Queue

`Poll` will maintain an internal readiness queue, represented as a
linked list. The linked list head pointer is an `AtomicPtr`. All of the
nodes in the linked list are owned by the `Poll` instance.

The type declarations are for illustration only. The actual
implementations will have some additional memory safety requirements.

```rust
struct Poll {
	readiness_queue: Arc<PollReadinessQueue>,
}

struct PollReadinessQueue {
	// All readiness nodes owned by the `Poll` instance. When the `Poll`
	// instance is freed, the list is walked and each Arc ref count is
	// decremented.
	head_all_nodes: Box<ReadinessNode>,

	// linked list of nodes that are pending some processing
	head_readiness: AtomicPtr<ReadinessNode>,

    // Hashed wheel timer for delayed readiness notifications
    readiness_wheel: Vec<AtomicPtr<ReadinessNode>>,
}

struct ReadinessNode {
	// Next node in ownership tracking queue
	next_all_nodes: Box<ReadinessNode>,
    // Used when the node is queued in the readiness linked list OR the
    // linked list for a hashed wheel slot.
	next_readiness: *mut ReadinessNode,
	// The Token used to register the `Evented` with `Poll`. This can change,
	// but only by calling `Poll` functions, so there will be no concurrency.
	token: Token,
	// The set of events to include in the notification on next poll
	events: AtomicUsize,
    // Tracks if the node is queued for readiness using the MSB, the
    // rest of the usize is the readiness delay.
	queued: AtomicUsize,
	// Both interest and opts can be mutated
	interest: Cell<EventSet>,
    // Poll opts
	opts: Cell<PollOpt>,
}

// Implements `Sync`, aka all functions are safe to call concurrently
struct Registration {
	node: *mut ReadinessNode,
	queue: Arc<PollReadinessQueue>,
}

struct MyEvented {
	registration: Option<Registration>,
}
```

#### Registration

When a `MyEvented` value is registered with the event loop, a new `Registration` value is obtained:

```rust
my_evented.registration = Some(Registration::new(poll, token, interest));
```

`Registration` will include the internal `EventSet::dropped()` event to
the interest.

#### Re-registration

A `Registration`'s `interest` & `PollOpt` can be changed by calling `Registration::update`:

```rust
// poll: &Poll
my_evented.registration.as_ref().unwrap()
	.update(poll, interest, opts);
```

The `Poll` reference will not be used but will ensure that `update` is
only called from a single thread (the thread that owns the `Poll`
reference). This allows safe mutation of `interest` and `opts` without
synchronization primitives.

`Registration` will include the internal `EventSet::dropped()` event to
the interest.

#### Triggering readiness notifications

Readiness can be updated using `Registration::set_readiness` and
`Registration::unset_readiness`. These can be called concurrently.
`set_readiness` adds the given events with the existing `Registration`
readiness. `unset_readiness` subtracts the given events from the
existing `Registration`.

```rust
my_evented.registration.as_ref().unwrap().set_readiness(EventSet::readable());
my_evented.registration.as_ref().unwrap().unset_readiness(EventSet::readable());
```

`Registration::set_readiness` ensures that the registration node is queued for processing. 

#### Delaying readiness

In order to support timeouts, `Registration` has the ability to schedule
readiness notifications using `Registration::delay_readiness(events,
timeout)`.

There is a big caveat. There is precise timing guarantee. A delayed
readiness event could be triggered much earlier than requested.  Also,
the readiness timer is coarse grained, so by default will be rounded to
100ms or so. The one guarantee is that the event will be triggered no
later than the requested timeout + the duration of a timer tick (100ms
by default).

#### Queuing `Registration` for processing

First, atomically update `Registration.queued`. Attempt to set the MSB.
Check the current delay value. If the requested delay is less than the
current, update the delayed portion of `queued`.

If the MSB was successfully set, then the current thread is responsible
for queuing the registration node (pseudocode):

```
loop {
	let ptr = PollReadinessQueue.readiness_head.get();
    ReadinessNode.next_readiness = ptr;

    if PollReadinessQueue.readiness_head.compare_and_swap(ptr, &ReadinessNode) {
        return;
    }
}
```

#### Dropping `Registration`

Processing a drop is handled by setting readiness to an internal
`Dropped` event:

```rust
fn drop(&mut self) {
    self.registration.as_ref().unwrap()
        .set_readiness(EventSet::dropped());
}
```

The `Registration` value itself does not own any data, so there is
nothing else to do.

#### Polling

On `Poll::poll()` the following happens:

Reset the events on self

```rust
self.events.clear();
```

Atomically take ownership of the readiness queue:

```rust
let ready_nodes = PollReadinessQueue.readiness_head.swap(ptr::null());
```

The dequeued nodes are processed.

```rust
for node in ready_nodes {
    // Mask the readiness info by the node's interest. This is needed to
    // support concurrent setting of readiness. Another thread may not
    // be aware of the latest interest value.
    let mut events = node.events.get() & node.interest;

    // Used to read the delay component of `Registration::queued`.
    let delay;

    if opts.is_edge() || events.is_empty() {
        // If the registration is edge, the node is always dequeued. If
        // it is level, we only dequeue the event when there are no
        // events (aka, no readiness). By not dequeing the event it will
        // be processed again next call to `poll`
        delay = unset_msb_and_read_delay_component(&node.queued);

        // Reload the events to ensure that we don't "lose" any
        // readiness notifications. Remember, it's ok to have
        // spurious notifications. 
        events = node.events.get() | node.interest;
    } else if !events.is_drop() {
        // Push the node back into the queue. This is done via a compare
        // and swap on `readiness_head`, pushing the node back to the
        // front.
        prepend(&ready_nodes, node);

        delay = read_delay_component(&node.queued);
    }

    if delay > 0 {
        node.update_delay_in_hashed_wheel(delay);
    } else {
        // The event will be fired immediately, if the node is currently
        // being delayed, remove it from the hashed wheel.
        if node.is_currently_in_hashed_wheel() {
            node.remove_from_hashed_wheel();
        }

        if events.is_drop() {
            // The registration is dropped. Unlink the node, freeing the
            // memory.
            node.unlink();
            continue;
        }

        if !events.is_empty() {
            // Track the event
            self.events.push_event(node.token, events);
        }
    }

}
```

The next step is to process all delayed readiness nodes that have
reached their timeout. The code for this is similar to the current timer
code.

### Integrating with `Selector`

The readiness queue described above is *not* to replace socket
notifications on epoll / kqueue / etc... It is to be used in conjuction.

To handle this, `PollReadinessQueue` will be able to wakup the selector.
This will be implemented in a similar fashion as the current channel
implementation. A pipe will be used to force the selector to wakeup.

The full logic of `poll` will look something like:

```rust
let has_pending = !readiness_queue.is_empty();

if has_pending {
    // Original timeout value is passed to the function...
    timeout = 0;
}

// Poll selector
selector.poll(&mut self.events, timeout);

// Process custom evented readiness queue as specified above.
```

### Implementing `mio::Channel`

`Channel` is a mpsc queue such that when messages are pushed onto the
channel, `Poll` is woken up and returns a readable readiness event for
the `Channel`. The specific queue will be supplied on creation of
`Channel`, allowing the user to choose the behavior around allocation
and capacity.

`Channel` will look something like:

```rust
struct Channel<Q> {
    queue: Q,

    // Poll registration
    registration: Option<Registration>,

    // Tracks the number of pending messages.
    pending: AtomicUsize,
}
```

When a new message is sent over the channel:

```rust
self.queue.push(msg);

let prev = self.pending.fetch_add(1);

if prev == 0 {
    // set readiness
    self.registration.as_ref().unwrap()
        .set_readiness(EventSet::readable());
}
```

When readiness is set, `Poll` will wake up with a readiness
notification. The user can now "poll" off of the channel. The
implementation of poll is something like:

```rust
self.queue.poll().map(|msg| {
    let first = self.pending.get();

    if first == 1 {
        self.registration.as_ref().unwrap()
            .unset_readiness(EventSet::readable());
    }

    let second = self.pending.fetch_sub(1);

    if first == 1 && second > 0 {
        // There still are pending messages, reset readiness
        self.registration.as_ref().unwrap()
            .set_readiness(EventSet::readable());
    }

    msg
})
```

### Implemented `Timer`

`Timer` is a delay queue. Messages are pushed onto it with a delay after
which the message can be "popped" from the queue. It is implemented
using a hashed wheel timer strategy which is ideal in situations where
large number of timeouts are required and the timer can use coarse
precision (by default, 100ms ticks).

The implementation is fairly straight forward. When a timeout is
requested, the message is stored in the `Timer` implementation and
`Registration::delay_readiness` is called with the timeout. There are
some potential optimizations, but those are out of scope for this
proposal.

### Windows

The readiness queue described in this proposal would replace the [current
windows specific
implementation](https://github.com/carllerche/mio/blob/master/src/sys/windows/selector.rs).
The proposal implementation would be more efficient as it avoids locking
as well as uses lighter weight data structures (mostly, linked lists vs.
vecs).

## Outstanding questions

The biggest outstanding question would be what to do about `EventLoop`.
If this proposal lands, then `EventLoop` becomes entirely a very light
shim around `Poll` that dispatches events to the appropriate handler
function.

The entire implementation would look something like:

```rust
pub fn run(&mut self, handler: &mut H) -> io::Result<()> {
    self.run = true;

    while self.run {
        self.poll.poll();

        for event in self.poll.events() {
            handler.ready(self, event.token(), event.kind());
        }

        handler.tick(self);
    }
}
```

It will also not be possible to maintain API compatibility.
`Handler::notify` and `Handler::timeout` will no longer exist as
`EventLoop` does not know the difference between those two types and
other `Evented` types that have notifications called through `ready`.

The options are:

- Update `EventLoop` to follow the new API and keep the minimal
  impelmentation.
- Get rid of `EventLoop` and make `Poll` the primary API
- Provide a hire level API via `EventLoop` that accepts allocations
  (though this would be post 1.0).

## Alternatives

It is possible to implement `Timer` and `Channel` as standalone types
without having to implement the readiness queue. For `Timer`, it would
require using timerfd on linux and a timer thread on other platforms.
The disadvanage here is minor for linux as syscalls can be reduced
significantly by only using `timerfd` to track the next timeout in the
`Timer` vs. every timeout in `Timer`.

However, on platforms that don't have `timerfd` available, a polyfill
will be needed. This can be done by creating a pipe and spawning a
thread. When a timeout is needed, send a request to the thread. The
thread writes a byte to the pipe after the timeout has expired. This has
overhead, but again it can be amortized by only using the thread/pipe
combo for the next timeout in `Timer` vs. every timeout. Though, there
may be some complication with this amoritization when registering the
`Timer` using level triggered notifications.

On the other hand. For `Channel`, a syscall would be needed for each
message enqueued and dequeued. The implementation would be to have a
pipe associated with the `Chanenl`. Each time a message is enqueued,
write a byte to the pipe. Whenever a message is dequeued, read a byte.
