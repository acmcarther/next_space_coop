use gaffer_udp::non_blocking::GafferSocket;

pub struct Network {
  pub socket: GafferSocket,
}


impl Network {
  pub fn new(port: u16) -> Network {
    let socket = GafferSocket::bind(("0.0.0.0", port)).unwrap();
    info!("Opened socket on port {}", port);
    Network { socket: socket }
  }
}
