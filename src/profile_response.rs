use crate::node::Node;
use crate::node::HandleError;
use std::net::SocketAddr;
use crate::peel::peel_u32;
use crate::node::DataRequestResolution;

pub struct ProfileResponse<'a> {
    pub token: u32,
    pub slice: &'a[u8],
}

impl<'a> ProfileResponse<'a> {
    pub fn serialize(&self) -> Vec<u8> {
        let capacity: usize = 5 + self.slice.len();
        let mut bs = Vec::with_capacity(capacity);
        
        bs.push(9);
        bs.extend_from_slice(&self.token.to_le_bytes()[..]);
        bs.extend_from_slice(self.slice);
        
        debug_assert_eq!(capacity, bs.len());
        
        bs
    }
    
    fn deserialize(body: &[u8]) -> Result<ProfileResponse, HandleError> {
        let (token, body) = peel_u32(body)?;
        let slice = body;
        
        Ok(ProfileResponse {
            token: token,
            slice: slice,
        })
    }
}

pub fn handle_profile_response(node: &mut Node, source: &SocketAddr, body: &[u8]) -> Result<(), HandleError> {
    let profile_response = ProfileResponse::deserialize(body)?;
    
    node.resolve_profile_request(profile_response.token, DataRequestResolution{bytes: profile_response.slice.to_vec()});
    
    Ok( () )
}