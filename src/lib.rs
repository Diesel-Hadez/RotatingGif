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
    pub delay: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GifData {
    pub width: u32,
    pub height: u32,
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
    let image = gif_me_hd::decoder::GifFile::new(bytes).unwrap();
    let width = image.logical_screen_descriptor.canvas_width;
    let height = image.logical_screen_descriptor.canvas_height;
    let gct: Vec<gif_me_hd::decoder::Pixel> = image.global_color_table.unwrap();
    let mut cur_color_table = gct;
    let mut frames: Vec<GifFrame> = Vec::new();

    for cur_frame in image.frames {
        cur_color_table = match cur_frame.local_color_table {
            Some(table) => table,
            None => cur_color_table,
        };
        let frame_data: Vec<Vec<u8>> = cur_frame.frame_indices
            .iter()
            .map(|x| *(cur_color_table.get(*x as usize).unwrap()))
            .map(|x| vec![x.red, x.green, x.blue, 255])
            .collect();

        let to_x_y  = |pos: u16, width: u16| {
            let x: u16 = pos % width;
            let y: u16 = (pos - x) / width;
            (x,y)
        };

        let to_x_y_global = |pos: u16| {
            to_x_y(pos, image.logical_screen_descriptor.canvas_width)
        };

        
        let frame = match frames.last() {
            Some(prev_frame) => {
                GifFrame {
                    data: prev_frame
                        .data
                        .chunks(4)
                        .enumerate()
                        .map(|(pos, val)| -> Vec<u8> {
                            // alert(&format!("{:#?}",val));
                            let (x, y) = to_x_y_global(pos as u16);
                            if x >= cur_frame.image_descriptor.left && 
                                x < cur_frame.image_descriptor.left
                                    + cur_frame.image_descriptor.width &&
                                y >= cur_frame.image_descriptor.top &&
                                y < cur_frame.image_descriptor.top 
                                    + cur_frame.image_descriptor.height {
                                        let local_x = x - cur_frame.image_descriptor.left;
                                        let local_y = y - cur_frame.image_descriptor.top;
                                        return frame_data
                                            .get((local_y*cur_frame.image_descriptor.width+local_x) as usize)
                                            .unwrap()
                                            .to_vec();
                                    }
                            val.to_vec()
                        })
                        .flatten()
                        .collect(),
                    delay: 0,
                }
            },
            None => {
                GifFrame {
                    data: frame_data
                        .into_iter()
                        .flatten()
                        .collect(),
                    delay: 0,
                }
            },
        };
    
        frames.push(frame);
    }
    

    let example = GifData { width: width as u32, height: height as u32, frames };


    Ok(serde_wasm_bindgen::to_value(&example)?)

}
