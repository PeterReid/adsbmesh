use crate::node::Node;
use crate::node::HandleError;
use std::net::SocketAddr;
use crate::peel::{peel_u32, peel_end};
use crate::node::PendingPartnershipResolution;
use std::time::Duration;
use std::time::Instant;

pub struct SubscribeAccept {
    pub partnering_id: u32,
    pub confirmation_nonce: u32,
}

impl SubscribeAccept {
    pub fn serialize(&self) -> Vec<u8> {
        const CAPACITY: usize = 9;
        let mut bs = Vec::with_capacity(CAPACITY);
        
        bs.push(2);
        bs.extend_from_slice(&self.partnering_id.to_le_bytes()[..]);
        bs.extend_from_slice(&self.confirmation_nonce.to_le_bytes()[..]);
        
        debug_assert_eq!(bs.len(), CAPACITY);
        bs
    }
    
    
    fn deserialize(body: &[u8]) -> Result<SubscribeAccept, HandleError> {
        let (partnering_id, body) = peel_u32(body)?;
        let (confirmation_nonce, body) = peel_u32(body)?;
        peel_end(&body)?;
        
        Ok(SubscribeAccept {
            partnering_id: partnering_id,
            confirmation_nonce: confirmation_nonce,
        })
    }
}

pub fn handle_subscribe_accept(node: &mut Node, _source: &SocketAddr, body: &[u8]) -> Result<(), HandleError> {
    let message = SubscribeAccept::deserialize(body)?;
    if let Some(accepted) = node.remove_pending_partnership_proposal(message.partnering_id, PendingPartnershipResolution::Accepted(message.confirmation_nonce)) {
        node.add_active_partnership(accepted);
        Ok( () )
    } else {
        Err( HandleError::DeclinedSubscriptionDoesNotExist )
    }
}
