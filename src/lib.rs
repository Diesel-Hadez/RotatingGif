use std::collections::HashMap;

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
    pub delay: u16,
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
    let mut cur_delay = 0;

    for cur_frame in image.frames {
        for extension in cur_frame.extensions {
            cur_delay = match extension {
                gif_me_hd::decoder::Extension::GraphicsControlExtension { reserved, disposal_method, user_input_flag, transparent_color_flag, delay_timer, transparent_color_index } => {
                    delay_timer
                },
                _ => {cur_delay}
            }
        }
        cur_color_table = match cur_frame.local_color_table {
            Some(table) => table,
            None => cur_color_table,
        };
        let frame_data: Vec<Vec<u8>> = cur_frame.frame_indices
            .iter()
            .map(|x| *(cur_color_table.get(*x as usize).unwrap()))
            .map(|x| vec![x.red, x.green, x.blue, 255])
            .collect();

        let to_x_y  = |pos: usize, width: u16| {
            let x: u16 = (pos % (width as usize)) as u16;
            let y: u16 = ((pos - (x as usize)) / (width as usize)) as u16;
            (x,y)
        };

        let to_x_y_global = |pos: usize| {
            to_x_y(pos, image.logical_screen_descriptor.canvas_width)
        };

        
        let frame = match frames.last() {
            Some(prev_frame) => {
                // alert(&format!("{}",prev_frame.data.len()));
                GifFrame {
                    // data: frame_data
                    //     .into_iter()
                    //     .flatten()
                    //     .collect(),
                    data: prev_frame
                        .data
                        .chunks(4)
                        .enumerate()
                        .collect::<Vec<(usize, &[u8])>>()
                        .into_iter()
                        .map(|(pos, val)| -> Vec<u8> {
                            let (x, y) = to_x_y_global(pos);
                            if x >= cur_frame.image_descriptor.left && 
                                x < cur_frame.image_descriptor.left
                                    + cur_frame.image_descriptor.width &&
                                y >= cur_frame.image_descriptor.top &&
                                y < cur_frame.image_descriptor.top 
                                    + cur_frame.image_descriptor.height {
                                        let local_x = x - cur_frame.image_descriptor.left;
                                        let local_y = y - cur_frame.image_descriptor.top;
                                        let end_result = vec![((local_x as f32 / cur_frame.image_descriptor.width as f32) * 255.0) as u8,
                                        0, 0, 255];
                            // alert(&format!("{} ({}, {}) {:#?} ",pos, local_x, local_y, end_result));
                            // alert(&format!("{} {} {} ", end_result[0], end_result[1], end_result[2]));
                                        // alert(&format!("{}, {}, {}, {}", local_x, local_y, cur_frame.image_descriptor.width, cur_frame.image_descriptor.height));
                                        // return end_result;
                                        // return vec![((local_x as f32 / cur_frame.image_descriptor.width as f32) * 255.0) as u8,
                                        // ((local_y as f32 / cur_frame.image_descriptor.height as f32) * 255.0) as u8, 0, 255];
                                        return frame_data
                                            .get(((local_y as usize)*(cur_frame.image_descriptor.width as usize)+(local_x as usize)) as usize)
                                            .unwrap()
                                            .to_vec();
                                    }
                            val.to_vec()
                        })
                        .flatten()
                        .collect(),
                    delay: cur_delay,
                }
            },
            None => {
                GifFrame {
                    data: frame_data
                        .into_iter()
                        .flatten()
                        .collect(),
                    delay: cur_delay,
                }
            },
        };
    
        frames.push(frame);
    }
    

    let example = GifData { width: width as u32, height: height as u32, frames };


    Ok(serde_wasm_bindgen::to_value(&example)?)

}
