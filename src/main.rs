mod subscribe;
mod subscribe_decline;
mod subscribe_accept;
mod subscribe_finalize;
mod data;
mod node;
mod peel;
mod profile_request;
mod profile_response;
mod partner_list_request;
mod partner_list_response;
mod seek;


use node::Node;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

fn main() {
    let source = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let node = Node::new("self.mynode.net".to_string());
    let node = Arc::new(Mutex::new(node));
    
    let thread_node = node.clone();
    thread::spawn(move || {
        seek::seek(thread_node)
    });
    
    node.lock().unwrap().handle_received_packet(&source, &[][..]).unwrap();
    
}
