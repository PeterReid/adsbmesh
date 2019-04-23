mod subscribe;
mod subscribe_decline;
mod node;
mod peel;
mod profile_request;
mod profile_response;
mod partner_list_request;
mod partner_list_response;


use node::Node;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let source = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    Node::new().handle_received_packet(&source, &[][..]).unwrap();
}
