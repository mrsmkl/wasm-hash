use tiny_keccak::{Keccak, Hasher};

pub fn main() {
    let mut input = vec![];
    for i in 0..10000 {
        input.push(123)
    }

    for i in 0..100000 {

        let mut hasher = Keccak::v256();

        // write input message
        hasher.update(&input[..]);

        let mut output = vec![0u8; 32];

        // read hash digest
        hasher.finalize(&mut output);
    }

}