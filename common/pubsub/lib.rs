extern crate specs;
extern crate itertools;

use std::marker::PhantomData;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::RwLockWriteGuard;
use std::any::Any;
use itertools::Itertools;

/// ////////////////////////////////////////////////////////////////////////////////////////////////
/// Publisher-Subscriber plugin for Specs ECS
///
/// The objective of this crate is to provide a safe API to emit and receive
/// events via the Specs
/// resource locking and unlocking mechanism.
///
/// TODO: Add more doc and overview
/// ////////////////////////////////////////////////////////////////////////////////////////////////

pub trait PubSubStore {
  fn fetch_publisher<'a, T: Clone + Any + Send + Sync>(&'a self) -> Publisher<'a, T>;
  fn fetch_subscriber<'a, T: Clone + Any + Send + Sync>(&'a self,
                                                        token: &SubscriberToken<T>)
                                                        -> Subscriber<'a, T>;
  fn register_subscriber<T: Clone + Any + Send + Sync>(&mut self) -> SubscriberToken<T>;
}

/// An indicator pointing to a specific subscriber reader that the holder has
/// access to.
pub struct SubscriberToken<T> {
  phantom_data: PhantomData<T>,
  reader_idx: u32,
}

struct PubSubRegistry {
  readers: HashMap<TypeId, u32>,
}


impl PubSubRegistry {
  fn new() -> PubSubRegistry {
    PubSubRegistry { readers: HashMap::new() }
  }
}

impl PubSubStore for specs::World {
  /// Attempt to retrieve the general publisher for this resource
  fn fetch_publisher<'a, T: Clone + Any + Send + Sync>(&'a self) -> Publisher<'a, T> {
    Publisher { senders: acquire_raw_containers::<T>(self) }
  }


  /// Attempt to retrieve the subscriber corresponding to this token
  fn fetch_subscriber<'a, T: Clone + Any + Send + Sync>(&'a self,
                                                        token: &SubscriberToken<T>)
                                                        -> Subscriber<'a, T> {

    Subscriber {
      receivers: acquire_raw_containers::<T>(self),
      idx: token.reader_idx,
    }
  }

  /// Allocate the subscriber a Receiver and provide them a token containing
  /// that receiver index.
  fn register_subscriber<T: Clone + Any + Send + Sync>(&mut self) -> SubscriberToken<T> {
    let idx = {
      let mut registry = acquire_registry(self);

      let type_id = TypeId::of::<T>();
      let mut idx = registry.readers.entry(type_id).or_insert(0);
      *idx += 1;
      idx.clone() - 1
    };

    acquire_or_install_raw_containers::<T>(self).push(NewContainer(Vec::new()));

    SubscriberToken {
      phantom_data: PhantomData,
      reader_idx: idx,
    }
  }
}

// Wrap builtins so they cannot be accessed outside this module
// This prevents accidental access by someone who happens to be storing Vecs
struct NewContainer<T: Clone + Any + Send + Sync>(pub Vec<T>);

impl<T: Clone + Any + Send + Sync> NewContainer<T> {
  pub fn get_mut_vec(&mut self) -> &mut Vec<T> {
    let &mut NewContainer(ref mut stuff) = self;
    stuff
  }
}

pub struct Publisher<'a, T: Any + Clone + Sync + Send> {
  senders: RwLockWriteGuard<'a, Vec<NewContainer<T>>>,
}

impl<'a, T: Any + Clone + Sync + Send> Publisher<'a, T> {
  pub fn push(&mut self, item: T) {
    self.senders.iter_mut().foreach(|mut s| s.get_mut_vec().push(item.clone()))
  }
}

pub struct Subscriber<'a, T: Any + Clone + Sync + Send> {
  receivers: RwLockWriteGuard<'a, Vec<NewContainer<T>>>,
  idx: u32,
}

impl<'a, T: Any + Clone + Sync + Send> Subscriber<'a, T> {
  pub fn pull(&mut self) -> &mut Vec<T> {
    self.receivers[self.idx as usize].get_mut_vec()
  }

  pub fn collected(mut self) -> Vec<T> {
    let mut contents = Vec::new();
    std::mem::swap(&mut contents, self.pull());
    contents
  }
}


fn acquire_registry<'a>(w: &'a mut specs::World) -> RwLockWriteGuard<'a, PubSubRegistry> {
  if !w.has_resource::<PubSubRegistry>() {
    w.add_resource::<PubSubRegistry>(PubSubRegistry::new());
  }
  w.write_resource::<PubSubRegistry>()
}

fn acquire_raw_containers<'a, T: Clone + Any + Send + Sync>
  (w: &'a specs::World)
   -> RwLockWriteGuard<'a, Vec<NewContainer<T>>> {
  w.write_resource::<Vec<NewContainer<T>>>()
}

fn acquire_or_install_raw_containers<'a, T: Clone + Any + Send + Sync>
  (w: &'a mut specs::World)
   -> RwLockWriteGuard<'a, Vec<NewContainer<T>>> {
  if !w.has_resource::<Vec<NewContainer<T>>>() {
    w.add_resource::<Vec<NewContainer<T>>>(Vec::new());
  }
  w.write_resource::<Vec<NewContainer<T>>>()
}

pub enum PubSubError {
  NoSuchType,
  NoSuchSubscriber,
}

#[cfg(test)]
mod test {
  use specs;
  use super::*;
  use itertools::Itertools;


  #[test]
  fn publishers_publish_and_subscribers_subscribe() {
    let mut world = specs::World::new();
    let stoken_1 = world.register_subscriber::<u32>();
    let stoken_2 = world.register_subscriber::<u32>();
    let stoken_3 = world.register_subscriber::<u32>();

    world.fetch_publisher::<u32>().push(1);
    assert_eq!(world.fetch_subscriber(&stoken_1).pull(), &vec![1]);
    assert_eq!(world.fetch_subscriber(&stoken_2).pull(), &vec![1]);

    world.fetch_subscriber(&stoken_1).pull().clear();


    world.fetch_publisher::<u32>().push(2);
    assert_eq!(world.fetch_subscriber(&stoken_1).pull(), &vec![2]);
    assert_eq!(world.fetch_subscriber(&stoken_2).pull(), &vec![1, 2]);
  }

}
