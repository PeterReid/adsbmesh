use crypto::poly1305::Poly1305;
use crypto::mac::Mac;

struct Data<'a> {
    partnering_id: u32,
    signature: [u8; 16],
    data: &'a [u8],
}

/// `DataSerializer` turns a data payload into a data packet, optimized for sending the same payload to
/// multiple recipients.
pub struct DataSerializer {
    buf: Vec<u8>
}

impl DataSerializer {
    pub fn new(sequence_number: u32, payload: &[u8]) -> DataSerializer {
        let capacity = 1 + 4 + 16 + 4 +  payload.len();
        let mut xs = Vec::with_capacity(capacity);
        xs.push(5); // Packet type
        xs.resize(1 + 4 + 16, 0); // Make space for the partnering_id and signature
        xs.extend_from_slice(&sequence_number.to_le_bytes()[..]);
        
        xs.extend_from_slice(payload);
        debug_assert!(xs.len() == capacity);
        
        DataSerializer {
            buf: xs
        }
    }

    pub fn serialize_for(&mut self, partnering_id: u32, key: &[u8; 32]) -> &[u8] {
        (&mut self.buf[1..1+4]).copy_from_slice(&partnering_id.to_le_bytes()[..]);
        
        let mut signer = Poly1305::new(&key[..]);
        signer.input(&self.buf[1+4+16..]);
        signer.raw_result(&mut self.buf[1+4..1+4+16]);
        
        &self.buf
    }
}
