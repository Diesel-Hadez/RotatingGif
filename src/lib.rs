use std::collections::HashMap;
use itertools::iproduct;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
}
#[derive(Serialize, Deserialize)]
pub struct GifFrame{
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct GifData {
    pub width: u32,
    pub height: u32,
    pub delay: u32,
    pub frames: Vec<GifFrame>,
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn get_stuff(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let image = gif_me_hd::decoder::GifFile::new(bytes);
    let width = 0..50;
    let height = 0..50;
    let image_iter = iproduct!(width, height);
    let frame_data: Vec<Vec<u8>> = image_iter
    .map(|(x,y)| {
        if x < 25 && y < 25 {
            vec![255, 0, 0, 255]
        }
        else if x < 25 && y > 25 {
            vec![0,255,0,255]
        }
        else if x > 25 && y < 25 {
            vec![0,0,255,255]
        }
        else if x > 25 && y > 25 {
            vec![255,255,0,255]
        }
        else {
            vec![255,255,255,255]
        }
    })
    .collect();

    let frame_data = frame_data.iter().flatten().cloned().collect();

    
    let first_frame = GifFrame {
        data: frame_data,
    };

    let example = GifData { width: 50, height: 50, delay: 0, frames: vec![first_frame] };


    Ok(serde_wasm_bindgen::to_value(&example)?)

}
