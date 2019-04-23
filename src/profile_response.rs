use crate::node::Node;
use crate::node::HandleError;
use std::net::SocketAddr;
use crate::peel::peel_u32;

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

