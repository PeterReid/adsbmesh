use crate::subscribe::handle_subscribe;
use crate::subscribe_decline::handle_subscribe_decline;
use crate::subscribe_accept::handle_subscribe_accept;
use crate::profile_request::handle_profile_request;
use crate::partner_list_request::handle_partner_list_request;
use std::net::SocketAddr;
use std::time::Instant;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::min;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use crate::subscribe::Subscribe;

pub enum PendingPartnershipResolution {
    Declined{retry_delay_seconds: u32},
    Accepted(u32),
    Timeout,
}


/// For what functionality belongs in the `Node` as opposed to some other module,
/// functionality is to go in the `Node` only if it must be grouped together to maintain
/// the `Node`'s invariants and does not do anything that blocks on network receive.
pub struct Node {

    /// Partnerships that we have proposed
    pending_partnerships: HashMap<u32, (Partnership, Sender<PendingPartnershipResolution>)>,
    
    /// Active partnerships are what we are actively communicating with
    active_partnerships: HashMap<u32, Partnership>,
    
    /// Inactive partnerships are understood to be temporarily offline
    inactive_partnerships: HashMap<u32, Partnership>,
    
    /// The `Node` tracks partnering IDs that are used in any way (pending, future, established)
    /// so that it can avoid collisions.
    used_partnering_ids: HashSet<u32>,
    
    profile: Vec<u8>,
    
    /// List of partnerships in the format expected by other nodes
    partner_list: Vec<u8>,
    
    /// Instructions for how to reach 
    contact_method: String,
    
    /// If the node is instructed not to re-send a partnership proposal, it is stored here
    partnership_proposal_not_before: HashMap<Addressable, Instant>,
}

pub type Addressable = String;

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
    pub fn new(contact_method: String) -> Node {
        Node {
            pending_partnerships: HashMap::new(),
            used_partnering_ids: HashSet::new(),
            profile: Vec::new(),
            partner_list: Vec::new(),
            active_partnerships: HashMap::new(),
            inactive_partnerships: HashMap::new(),
            contact_method: contact_method,
            partnership_proposal_not_before: HashMap::new(),
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
    
    fn make_partnership(&mut self, who: Addressable) -> Partnership {
        Partnership{
            address: who,
            resolved_address: None,
            key: self.random_key(),
            id: self.unused_partnering_id(),
        }
    }
    
    pub fn create_partnership_proposal(&mut self, who: Addressable) -> (u32, Vec<u8>, Receiver<PendingPartnershipResolution>) {
        let (sender, receiver) = channel();
    
        let p = self.make_partnership(who);
        let id = p.id;
        
        let message = Subscribe::new(id, &p.key, self.contact_method.as_bytes()).serialize();
        self.used_partnering_ids.insert(id);
        self.pending_partnerships.insert(id, (p, sender));
        
        (id, message, receiver)
    }
    
    pub fn remove_pending_partnership_proposal(&mut self, partnering_id: u32, reason: PendingPartnershipResolution) -> Option<Partnership> {
        if let Some((p, resolution_sender)) = self.pending_partnerships.remove(&partnering_id) {
            self.used_partnering_ids.remove(&partnering_id);
            let _ = resolution_sender.send(reason);
            Some(p)
        } else {
            None
        }
    }

    pub fn get_partnership(&self, partnership_id: u32) -> Option<&Partnership> {
        self.active_partnerships.get(&partnership_id).or_else(|| {
            self.inactive_partnerships.get(&partnership_id)
        })
    }
    
    pub fn update_partner_list(&mut self) {
        let mut partner_list = Vec::new();
        for partner in self.active_partnerships.values() {
            partner_list.extend(partner.address.as_bytes());
            partner_list.push(0);
        }
    }

    pub fn send(&self, destination: &SocketAddr, packet: &[u8]) {
        
    }
    
    pub fn handle_received_packet(&mut self, source: &SocketAddr, packet: &[u8]) -> Result<(), HandleError> {
        let (packet_type, body) = packet_type_and_body(packet)?;
        
        let handlers = [
            handle_subscribe,
            handle_subscribe_decline,
            handle_subscribe_accept,
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
    
    pub fn active_partnership_count(&self) -> usize {
        self.active_partnerships.len()
    }
    
    pub fn add_active_partnership(&mut self, partnership: Partnership) {
        let id = partnership.id;
        self.active_partnerships.insert(id, partnership);
        self.used_partnering_ids.insert(id);
    }
    
    pub fn delay_partnership_proposal_until(&mut self, addressable: Addressable, when: Instant) {
        self.partnership_proposal_not_before.insert(addressable, when);
    }
}
