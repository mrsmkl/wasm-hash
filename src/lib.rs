mod utils;

use wasm_bindgen::prelude::*;
use tiny_keccak::{Keccak, Hasher};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn read_buffer(idx: i32) -> i32;
    fn setlen(idx: i32);
    fn getlen() -> i32;
    fn write_buffer(idx: i32, c: i32);
    fn usegas(gas: i32);
    fn rvec(ptr: *mut u8, idx: i32, len: i32);
    fn wvec(ptr: *mut u8, idx: i32, len: i32);
}

#[wasm_bindgen]
pub fn test() -> u32 {
    let input_len = getlen();
    let mut input = vec![0; input_len as usize];

    rvec(input.as_mut_ptr(), 0, input_len);

    usegas(input_len / 10 + 1);

    let mut hasher = Keccak::v256();

    // write input message
    hasher.update(&input[..]);

    let mut output = vec![0u8; 32];

    // read hash digest
    hasher.finalize(&mut output);

    /*
    for i in 0..32 {
        write_buffer(i, input[i as usize] as i32)
    };
    for i in 0..32 {
        write_buffer(i, output[i as usize] as i32)
    };
    */
    wvec(output.as_mut_ptr(), 0, 32);
    setlen(32);

    0

}

/*
#[wasm_bindgen]
pub fn test() -> u32 {
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

    0


}
*/
