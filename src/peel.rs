
use crate::node::HandleError;

pub fn peel_u32(xs: &[u8]) -> Result<(u32, &[u8]), HandleError> {
    if xs.len() < 4 {
        return Err(HandleError::PacketTruncated);
    }
    
    let (first_4, rest) = xs.split_at(4);
    Ok( (u32::from_le_bytes( [ first_4[0], first_4[1], first_4[2], first_4[3] ] ), rest) )
}

pub fn peel_slice(xs: &[u8], len: usize) -> Result<(&[u8], &[u8]), HandleError> {
    if xs.len() < len {
        return Err(HandleError::PacketTruncated);
    }
    
    Ok(xs.split_at(len))
}

pub fn peel_end(xs: &[u8]) -> Result<(), HandleError> {
    if xs.len() != 0 {
        Err(HandleError::PacketContinuedUnexpectedly)
    } else {
        Ok( () )
    }
}