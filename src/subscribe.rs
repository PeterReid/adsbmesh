use crate::node::Node;
use crate::node::HandleError;
use crate::subscribe_decline::SubscribeDecline;
use std::net::SocketAddr;
use crate::peel::{peel_u32, peel_slice};

struct Subscribe<'a> {
    partnering_id: u32,
    key: &'a [u8],
    contact_method: &'a [u8],
}

impl<'a> Subscribe<'a> {
    fn deserialize(body: &[u8]) -> Result<Subscribe, HandleError> {
        let (partnering_id, body) = peel_u32(body)?;
        let (key, body) = peel_slice(body, 32)?;
        let contact_method = body;
        
        Ok(Subscribe {
            partnering_id: partnering_id,
            key: key,
            contact_method: contact_method,
        })
    }
}

pub fn handle_subscribe(node: &mut Node, source: &SocketAddr, body: &[u8]) -> Result<(), HandleError> {   
    let message = Subscribe::deserialize(body)?;
    
    if node.get_partnership(message.partnering_id).is_some() {
        // We are being asked to establish a partnership for an ID that is already used.
        // This is probably an unfortunate and rare coincidence.
        // We will ask the sender to retry again immediately, which amounts to just re-randomizing the proposed partnership id.
        node.send(
            source,
            &SubscribeDecline{
                partnering_id: message.partnering_id,
                retry_delay_seconds: 0,
            }.serialize()
        );
        return Ok( () );
    }
    
    
    
    Ok( () )
}