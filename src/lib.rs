use std::collections::HashMap;

use gif_me_hd::decoder::Pixel;
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
    let width = image.logical_screen_descriptor.canvas_width as u32;
    let height = image.logical_screen_descriptor.canvas_height as u32;
    let mut cur_color_table = image.global_color_table.clone();
    let mut frames: Vec<GifFrame> = Vec::new();
    let mut cur_delay = 0;
    let mut cur_transparent_index = 0;
    let mut last_frame: Vec<Vec<u8>> = vec![];
    for _ in 0..width*height {
        last_frame.push(vec![0,0,0,0]);
    }

    for cur_frame in image.frames {
        for extension in cur_frame.extensions {
            match extension {
                gif_me_hd::decoder::Extension::GraphicsControlExtension { reserved, disposal_method, user_input_flag, transparent_color_flag, delay_timer, transparent_color_index } => {
                    cur_delay = delay_timer;
                    cur_transparent_index = transparent_color_index;
                },
                _ => {}
            }
        }
        cur_color_table = match &cur_frame.local_color_table {
            Some(table) => cur_frame.local_color_table.clone(),
            None => cur_color_table,
        };
        let frame_data: Vec<Vec<u8>> = cur_frame.frame_indices
            .iter()
            .map(|x| {
                if *x == cur_transparent_index{
                    return vec![0,0,0,0];
                }
                else {
                    let p = *(cur_color_table.as_ref().unwrap().get(*x as usize).unwrap());
                    return vec![p.red, p.green, p.blue, 255];
                }
            })https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial
            .collect();

        let to_x_y  = |pos: usize, width: u16| {
            let x: u16 = (pos % (width as usize)) as u16;
            let y: u16 = ((pos - (x as usize)) / (width as usize)) as u16;
            (x,y)
        };

        let to_x_y_global = |pos: usize| {
            to_x_y(pos, image.logical_screen_descriptor.canvas_width)
        };

        
        let frame_data =                 // alert(&format!("{}",prev_frame.data.len()));
                    last_frame 
                        .iter()
                        .enumerate()
                        .collect::<Vec<(usize, &Vec<u8>)>>()
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
                                        return frame_data
                                            .get(((local_y as usize)*(cur_frame.image_descriptor.width as usize)+(local_x as usize)) as usize)
                                            .unwrap()
                                            .to_vec();
                                    }
                            vec![0,0,0,0]
                        })
                    .collect::<Vec<Vec<u8>>>();
        last_frame = frame_data.clone();
        let frame = GifFrame {
            data: frame_data
                .iter()
                        .flatten()
                        .map(|x| *x)
                        .collect(),
            delay: cur_delay,
        };
    
        frames.push(frame);
    }
    

    let example = GifData { width: width as u32, height: height as u32, frames };


    Ok(serde_wasm_bindgen::to_value(&example)?)

}
