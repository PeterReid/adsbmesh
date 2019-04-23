use crate::subscribe::handle_subscribe;
use crate::subscribe_decline::handle_subscribe_decline;
use crate::profile_request::handle_profile_request;
use crate::partner_list_request::handle_partner_list_request;
use std::net::SocketAddr;
use std::time::Instant;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::min;

pub struct Node {

    /// Partnerships that we have proposed
    pending_partnerships: HashMap<u32, Partnership>,
    
    /// Later partnerships
    future_partnership_proposals: BTreeMap<Instant, Vec<Partnership>>,
    
    /// The `Node` tracks partnering IDs that are used in any way (pending, future, established)
    /// so that it can avoid collisions.
    used_partnering_ids: HashSet<u32>,
    
    profile: Vec<u8>,
    
    /// List of partnerships in the format expected by other nodes
    partner_list: Vec<u8>,
    
    
}

type Addressable = String;

pub struct Partnership {
    pub address: Addressable,
    pub resolved_address: Option<SocketAddr>,
    pub key: [u8; 32],
    pub id: u32,
}

#[derive(Debug)]
pub enum HandleError {
    MissingPacketType,
    InvalidPacketType,
    PacketTruncated,
    PacketContinuedUnexpectedly,
    DeclinedSubscriptionDoesNotExist,
}

fn packet_type_and_body(packet: &[u8]) -> Result<(u8, &[u8]), HandleError> {
    if packet.len() == 0 {
        return Err(HandleError::MissingPacketType);
    }
    let (first, rest) = packet.split_at(1);
    Ok((first[0], rest))
}

fn slice_of(source: &[u8], start: u32, len: usize) -> &[u8] {
    let start_usize = start as usize;
    if (start_usize as u32) != start {
        return &source[0..0];
    }
    if start_usize > source.len() {
        return &source[0..0];
    }
    
    let end = start_usize.checked_add(len).unwrap_or(source.len());
    
    &source[start_usize..min(source.len(), end)]
}


impl Node {
    pub fn new() -> Node {
        Node {
            pending_partnerships: HashMap::new(),
            future_partnership_proposals: BTreeMap::new(),
            used_partnering_ids: HashSet::new(),
            profile: Vec::new(),
            partner_list: Vec::new(),
        }
    }

    fn random_key(&mut self) -> [u8; 32] {
        [0u8; 32]
    }
    
    fn random_u32(&mut self) -> u32 {
        0
    }
    
    fn unused_partnering_id(&mut self) -> u32 {
        loop {
            let id = self.random_u32();
            if !self.used_partnering_ids.contains(&id) {
                return id;
            }
        }
    }
    
    pub fn remove_pending_partnership_proposal(&mut self, partnering_id: u32) -> Option<Partnership> {
        if let Some(p) = self.pending_partnerships.remove(&partnering_id) {
            self.used_partnering_ids.remove(&partnering_id);
            Some(p)
        } else {
            None
        }
    }
    
    pub fn add_future_partnership_proposal(&mut self, when: Instant, who: Addressable) {
        let p = Partnership{
            address: who,
            resolved_address: None,
            key: self.random_key(),
            id: self.unused_partnering_id(),
        };
        self.used_partnering_ids.insert(p.id);
        self.future_partnership_proposals.entry(when).or_insert(Vec::new()).push(p);
    }

    pub fn get_partnership(&self, partnership_id: u32) -> Option<&Partnership> {
        None
    }

    pub fn send(&self, destination: &SocketAddr, packet: &[u8]) {
    }
    
    pub fn handle_received_packet(&mut self, source: &SocketAddr, packet: &[u8]) -> Result<(), HandleError> {
        let (packet_type, body) = packet_type_and_body(packet)?;
        
        let handlers = [
            handle_subscribe,
            handle_subscribe_decline,
            handle_profile_request,
            handle_partner_list_request,
        ];
        
        let handler = handlers.get(packet_type as usize).ok_or(HandleError::InvalidPacketType)?;
        
        handler(self, source, body)
    }
    
    pub fn extract_profile_slice(&mut self, start: u32, len: usize) -> &[u8] {
        slice_of(&self.profile, start, len)
    }
    
    pub fn extract_partner_list_slice(&mut self, start: u32, len: usize) -> &[u8] {
        slice_of(&self.partner_list, start, len)
    }
}
