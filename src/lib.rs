extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    type KV;

    #[wasm_bindgen(static_method_of = KV)]
    pub async fn get(s: String) -> JsValue;
}

#[wasm_bindgen]
pub fn run_arr() -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u32> = (1..51).collect();
    numbers.shuffle(&mut rng);
    
    println!("Shuffled: {:?}", numbers);

    numbers
}

#[wasm_bindgen]
pub fn run_str() -> JsValue {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u32> = (1..51).collect();
    numbers.shuffle(&mut rng);
    
    println!("Shuffled: {:?}", numbers);

    let mut str_vec: Vec<String> = vec!();
    for chunk in numbers.chunks(5) {
        println!("Chunk: {:?}", chunk.clone());

        let mut nums = format!("{:?}", chunk);
        nums.retain(|c| c != '[' && c != ']' && c != ' ');

        str_vec.push(nums);
    }

    let result = serde_wasm_bindgen::to_value(&str_vec).unwrap();
    result
}

#[wasm_bindgen]
pub fn run_encode(data: &str) -> String {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(data.as_bytes())
        .expect("could not compress");
    let compressed_bytes = e.finish();
    let s = base64::encode(&compressed_bytes.unwrap());
    s
}

#[wasm_bindgen]
pub fn run_decode(data: &str) -> String {
    let compressed_bytes = base64::decode(&data).unwrap();
    let mut d = GzDecoder::new(&compressed_bytes[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    s
}
