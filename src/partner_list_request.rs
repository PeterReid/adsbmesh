use crate::node::Node;
use crate::node::HandleError;
use crate::partner_list_response::PartnerListResponse;
use std::net::SocketAddr;
use crate::peel::{peel_u32};

pub struct PartnerListRequest {
    pub token: u32,
    pub start_index: u32,
    pub requested_len: usize,
}

impl PartnerListRequest {
    pub fn serialize(&self) -> Vec<u8> {
        let capacity: usize = 9 + self.requested_len;
        let mut bs = Vec::with_capacity(capacity);
        
        bs.push(8);
        bs.extend_from_slice(&self.token.to_le_bytes()[..]);
        bs.extend_from_slice(&self.requested_len.to_le_bytes()[..]);
        bs.resize(capacity, 0);
        
        bs
    }
    
    fn deserialize(body: &[u8]) -> Result<PartnerListRequest, HandleError> {
        let (token, body) = peel_u32(body)?;
        let (start_index, body) = peel_u32(body)?;
        let requested_len = body.len();
        
        Ok(PartnerListRequest {
            token: token,
            start_index: start_index,
            requested_len: requested_len,
        })
    }
}

pub fn handle_partner_list_request(node: &mut Node, source: &SocketAddr, body: &[u8]) -> Result<(), HandleError> {
    let partner_list_request = PartnerListRequest::deserialize(body)?;
    let slice = node.extract_partner_list_slice(partner_list_request.start_index, partner_list_request.requested_len);
    let response = PartnerListResponse{
        token: partner_list_request.token,
        slice: slice,
    }.serialize();
    
    node.send(source, &response);
    
    Ok( () )
}