use crate::node::Node;
use crate::node::HandleError;
use crate::profile_response::ProfileResponse;
use std::net::SocketAddr;
use crate::peel::{peel_u32};

pub struct ProfileRequest {
    pub token: u32,
    pub start_index: u32,
    pub requested_len: usize,
}

impl ProfileRequest {
    pub fn serialize(&self) -> Vec<u8> {
        let capacity: usize = 9 + self.requested_len;
        let mut bs = Vec::with_capacity(capacity);
        
        bs.push(8);
        bs.extend_from_slice(&self.token.to_le_bytes()[..]);
        bs.extend_from_slice(&self.requested_len.to_le_bytes()[..]);
        bs.resize(capacity, 0);
        
        bs
    }
    
    fn deserialize(body: &[u8]) -> Result<ProfileRequest, HandleError> {
        let (token, body) = peel_u32(body)?;
        let (start_index, body) = peel_u32(body)?;
        let requested_len = body.len();
        
        Ok(ProfileRequest {
            token: token,
            start_index: start_index,
            requested_len: requested_len,
        })
    }
}

pub fn handle_profile_request(node: &mut Node, source: &SocketAddr, body: &[u8]) -> Result<(), HandleError> {
    let profile_request = ProfileRequest::deserialize(body)?;
    let slice = node.extract_profile_slice(profile_request.start_index, profile_request.requested_len);
    let response = ProfileResponse{
        token: profile_request.token,
        slice: slice,
    }.serialize();
    
    node.send(source, &response);
    
    Ok( () )
}