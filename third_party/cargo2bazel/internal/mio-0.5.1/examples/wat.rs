extern crate mio;

pub fn main() {
    use mio::*;
    use mio::tcp::*;

    // Construct a new `Poll` handle
    let mut poll = Poll::new().unwrap();

    // Connect the stream
    let stream = TcpStream::connect(&"173.194.33.80:80".parse().unwrap()).unwrap();

    // Register the stream with `Poll`
    poll.register(&stream, Token(0), EventSet::all(), PollOpt::edge()).unwrap();

    // Wait for the socket to become ready
    poll.poll(None).unwrap();
}
