use crate::node::Node;
use crate::node::HandleError;
use std::net::SocketAddr;
use crate::peel::{peel_u32, peel_end};
use crate::node::PendingPartnershipResolution;

pub struct SubscribeDecline {
    pub partnering_id: u32,
    pub retry_delay_seconds: u32,
}

impl SubscribeDecline {
    pub fn serialize(&self) -> Vec<u8> {
        const CAPACITY: usize = 9;
        let mut bs = Vec::with_capacity(CAPACITY);
        
        bs.push(1);
        bs.extend_from_slice(&self.partnering_id.to_le_bytes()[..]);
        bs.extend_from_slice(&self.retry_delay_seconds.to_le_bytes()[..]);
        
        debug_assert_eq!(bs.len(), CAPACITY);
        bs
    }
    
    
    fn deserialize(body: &[u8]) -> Result<SubscribeDecline, HandleError> {
        let (partnering_id, body) = peel_u32(body)?;
        let (retry_delay_seconds, body) = peel_u32(body)?;
        peel_end(&body)?;
        
        Ok(SubscribeDecline {
            partnering_id: partnering_id,
            retry_delay_seconds: retry_delay_seconds,
        })
    }
}

pub fn handle_subscribe_decline(node: &mut Node, _source: &SocketAddr, body: &[u8]) -> Result<(), HandleError> {
    let message = SubscribeDecline::deserialize(body)?;
    if let Some(declined) = node.remove_pending_partnership_proposal(message.partnering_id, PendingPartnershipResolution::Declined{retry_delay_seconds: message.retry_delay_seconds}) {
        Ok( () )
    } else {
        Err( HandleError::DeclinedSubscriptionDoesNotExist )
    }
}
