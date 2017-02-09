use lite::server::LiteServer;

#[test]
fn lite_server_can_start_without_exploding() {
  let client = LiteServer::new("127.0.0.1:28001");
}
