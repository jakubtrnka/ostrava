#[derive(Clone, Debug)]
struct BlockHeader {
    version: u32,
    prev_hash: Vec<u8>,
    merkle_root: Vec<u8>,
    ntime: u32,
    nbits: u32,
    nonce: u32,
}

impl BlockHeader {
    fn hash(&self) -> [u8; 32] {
        let mut serialized = Vec::new();
        serialized.extend_from_slice(&self.version.to_le_bytes());
        serialized.extend_from_slice(&self.prev_hash);
        serialized.extend_from_slice(&self.merkle_root);
        serialized.extend_from_slice(&self.ntime.to_le_bytes());
        serialized.extend_from_slice(&self.nbits.to_le_bytes());
        serialized.extend_from_slice(&self.nonce.to_le_bytes());

        let hash = ring::digest::digest(&ring::digest::SHA256, &serialized);
        let hash = ring::digest::digest(&ring::digest::SHA256, hash.as_ref());

        let mut output = [0; 32];
        output.copy_from_slice(hash.as_ref());
        output
    }
}

fn main() {

    let mut block_hdr = BlockHeader {
        version: 0x2bc2a000,
        prev_hash: hex::decode("00000000000000000003f133670336cf1a0b54bc780bef4b462ada000dd31ef1").unwrap().into_iter().rev().collect(),
        merkle_root: hex::decode("0caed4021328d2733d4f3106d19bbf6e9148d74cc31ac172d8164242ed12b9c5").unwrap().into_iter().rev().collect(),
        ntime: 1680037806,
        nbits: 0x1706023e,
        nonce: 0x9a47204a
    };

    let (sender, reader) = std::sync::mpsc::channel::<u32>();
    let threads = 16_u32;
    for offset in 0..threads {
        let mut hdr_clone = block_hdr.clone();
        let sender_clone = sender.clone();
        std::thread::spawn(move || {
            hdr_clone.nonce = offset;
            loop {
                let result = hdr_clone.hash();
                let mut last_word = [0_u8; 4];
                last_word.copy_from_slice(&result[28..32]);
                if u32::from_le_bytes(last_word) < 128 {
                    sender_clone.send(hdr_clone.nonce).unwrap();
                }
                if let Some(new_nonce) = hdr_clone.nonce.checked_add(threads) {
                    hdr_clone.nonce = new_nonce;
                } else {
                    break;
                }
            }
        });
    }
    drop(sender);

    while let Ok(nonce) = reader.recv() {
        block_hdr.nonce = nonce;

        println!("nonce: {}, hash: {}", block_hdr.nonce, hex::encode(block_hdr.hash()));
    }
}
