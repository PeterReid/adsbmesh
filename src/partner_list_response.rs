use crate::node::Node;
use crate::node::HandleError;
use std::net::SocketAddr;
use crate::peel::peel_u32;
use crate::node::DataRequestResolution;

pub struct PartnerListResponse<'a> {
    pub token: u32,
    pub slice: &'a[u8],
}

impl<'a> PartnerListResponse<'a> {
    pub fn serialize(&self) -> Vec<u8> {
        let capacity: usize = 10 + self.slice.len();
        let mut bs = Vec::with_capacity(capacity);
        
        bs.push(9);
        bs.extend_from_slice(&self.token.to_le_bytes()[..]);
        bs.extend_from_slice(self.slice);
        
        debug_assert_eq!(capacity, bs.len());
        
        bs
    }
    
    fn deserialize(body: &[u8]) -> Result<PartnerListResponse, HandleError> {
        let (token, body) = peel_u32(body)?;
        let slice = body;
        
        Ok(PartnerListResponse {
            token: token,
            slice: slice,
        })
    }
}

pub fn handle_partner_list_response(node: &mut Node, source: &SocketAddr, body: &[u8]) -> Result<(), HandleError> {
    let partner_list_response = PartnerListResponse::deserialize(body)?;
    
    node.resolve_partner_list_request(partner_list_response.token, DataRequestResolution{bytes: partner_list_response.slice.to_vec()});
    
    Ok( () )
}