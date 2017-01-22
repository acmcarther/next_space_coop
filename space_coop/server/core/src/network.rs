use gaffer_udp::non_blocking::GafferSocket;

pub struct Network {
  socket: GafferSocket
}


impl Network {
  pub fn new(port: u16) -> Network {
    Network {
      socket: GafferSocket::bind(("0.0.0.0", port)).unwrap()
    }
  }
}
