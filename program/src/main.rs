// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let from = sp1_zkvm::io::read::<[u8; 20]>();
    let to = sp1_zkvm::io::read::<[u8; 20]>();
    let amount = sp1_zkvm::io::read::<u64>();
    let hash = sp1_zkvm::io::read::<[u8; 32]>();

    let is_valid = from == to && amount == amount;
    sp1_zkvm::io::commit(&is_valid);
}
