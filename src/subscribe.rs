use crate::node::Node;
use crate::node::HandleError;
use crate::subscribe_decline::SubscribeDecline;
use std::net::SocketAddr;
use crate::peel::{peel_u32, peel_slice};

pub struct Subscribe<'a> {
    partnering_id: u32,
    key: &'a [u8],
    contact_method: &'a [u8],
}


impl<'a> Subscribe<'a> {
    pub fn new(partnering_id: u32, key: &'a [u8; 32], contact_method: &'a [u8]) -> Subscribe<'a> {
        Subscribe{
            partnering_id: partnering_id,
            key: &key[..],
            contact_method: contact_method
        }
    }

    pub fn deserialize(body: &[u8]) -> Result<Subscribe, HandleError> {
        let (partnering_id, body) = peel_u32(body)?;
        let (key, body) = peel_slice(body, 32)?;
        let contact_method = body;
        
        Ok(Subscribe {
            partnering_id: partnering_id,
            key: key,
            contact_method: contact_method,
        })
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        let capacity: usize = 1 + 4 + 32 + self.contact_method.len();
        let mut bs = Vec::with_capacity(capacity);
        
        bs.push(1);
        bs.extend_from_slice(&self.partnering_id.to_le_bytes()[..]);
        bs.extend_from_slice(self.key);
        bs.extend_from_slice(self.contact_method);
        
        bs
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