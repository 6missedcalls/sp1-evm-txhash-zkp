// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use tiny_keccak::{Hasher, Keccak};

pub fn main() {
    let rlp_len = sp1_zkvm::io::read::<usize>();
    let mut rlp_bytes = Vec::with_capacity(rlp_len);
    for _ in 0..rlp_len {
        rlp_bytes.push(sp1_zkvm::io::read::<u8>());
    }

    let claimed_hash = sp1_zkvm::io::read::<[u8; 32]>();
    let mut keccak256 = Keccak::v256();
    keccak256.update(&rlp_bytes);
    let mut computed_hash = [0u8; 32];
    keccak256.finalize(&mut computed_hash);

    let is_valid = claimed_hash == computed_hash;
    sp1_zkvm::io::commit(&is_valid);
}
