



// https://noir-lang.org/index.html
// https://github.com/rust-cc/awesome-cryptography-rust#zero-knowledge-proofs

use crate::*;



pub async fn consensus_on(block: &mut Block) -> bool{

    // TODO - current_block.generate_merkle_root_hash() 
    // TODO - current_block.generate_hash()
    // ...
    
    block.is_valid = true; //// set this to true if nodes reached a consensus on this block  

    todo!()
    
}