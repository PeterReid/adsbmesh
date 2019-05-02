use crate::node::Node;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crate::node::Addressable;
use std::sync::mpsc::Receiver;
use std::net::ToSocketAddrs;
use crate::node::PendingPartnershipResolution;
use crate::subscribe_finalize::SubscribeFinalize;
use std::time::Instant;


fn needs_more_partners(node: &Mutex<Node>) -> bool {
    const WANTED_PARTNERS: usize = 20;
    node.lock().unwrap().active_partnership_count() < WANTED_PARTNERS
}

fn get_partner_candidate(node: &Mutex<Node>) -> Option<Addressable> {
    None
}

fn request_partnership(node: &Mutex<Node>, who: Addressable) -> bool {
    if let Some(socket_addr) = who.to_socket_addrs().ok().and_then(|mut socket_addrs| socket_addrs.next()) { // to_socket_addrs can block on network receive and so does not belong in the `Node`
        let mut try_number = 0;
        const MAX_TRIES: u8 = 4;
        while try_number < MAX_TRIES {
            try_number += 1;
            let (potential_id, message, result_receiver) = node.lock().unwrap().create_partnership_proposal(who.clone());
            node.lock().unwrap().send(&socket_addr, &message);
            match result_receiver.recv_timeout(Duration::from_secs(10)) {
                Ok(PendingPartnershipResolution::Accepted(confirmation_nonce)) => {
                    // The `Node` will have removed the partnership proposed and promoted it to a partnership.
                    
                    let confirmation_message = SubscribeFinalize{
                        partnering_id: potential_id,
                        confirmation_nonce: confirmation_nonce
                    }.serialize();
                    
                    node.lock().unwrap().send(&socket_addr, &confirmation_message);
                    
                    return true;
                }
                Ok(PendingPartnershipResolution::Declined{retry_delay_seconds}) => {
                    if retry_delay_seconds > 0 {
                        let retry_time = Instant::now() + Duration::from_secs(retry_delay_seconds as u64);
                        node.lock().unwrap().delay_partnership_proposal_until(who.clone(), retry_time);
                        return false;
                    } else {
                        // We've been asked to retry with a delay of 0 seconds. Most likely, this is because a partnering id is already in use for the other node.
                        // Loop around so we try again immediately.
                    }
                }
                Ok(PendingPartnershipResolution::Timeout) => {
                    return false;
                }
                Err(_) => {
                    // recv_timeout timed out. Tell the node to abandon it.
                    node.lock().unwrap().remove_pending_partnership_proposal(potential_id, PendingPartnershipResolution::Timeout);
                    return false;
                }
            }
        }
        
        // We tried lots of times and kept getting told to retry immediately. Don't do that forever.
        false
    } else {
        false
    }
}

pub fn seek(node: Arc<Mutex<Node>>) {
    loop {
        if needs_more_partners(&node) {
            if let Some(addressable) = get_partner_candidate(&node) {
                request_partnership(&node, addressable);
            }
        }
        
        sleep(Duration::from_secs(30))
    }
}