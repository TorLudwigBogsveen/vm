/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use eframe::epaint::ColorImage;

use crate::color::Color;
pub struct FrameBuffer {
    width: u16,
    height: u16,
    data: Vec<u8>,
}

impl FrameBuffer {
    pub fn new(raw_data: &[u8], width: u16, height: u16, colors: &[Color]) -> FrameBuffer {

        let mut frame_buffer = FrameBuffer {
            width,
            height,
            data: Vec::new(),
        };

        let mut n_bits_per_color = 1;

        for i in 0..64 {
            if (colors.len()) & (1 << (63-i)) > 0 {
                //println!("föö");
                n_bits_per_color = 64 - i;
                if colors.len() % 2 == 0 {
                    n_bits_per_color -= 1;
                }
                break;
            }
        }

        //println!("nd: {}", n_bits_per_color);

        let mut bit_index = 0;

        for _ in 0..(width as usize * height as usize) {
            let mut color_index = 0;
            for i in 0..n_bits_per_color {
                let abs_bit = bit_index+i;
                let bit = abs_bit % 8;
                let index = (abs_bit - bit) / 8;
                color_index += ((raw_data[index] & (1 << bit)) >> bit) << i;
            } 

            //println!("n: {}", color_index);

            bit_index += n_bits_per_color;
            let (r, g, b, _a) = <(u8, u8, u8, u8)>::from(colors[color_index as usize % colors.len()]);
            frame_buffer.data.push(r);
            frame_buffer.data.push(g);
            frame_buffer.data.push(b);
        }

        frame_buffer
    }
}

impl From<FrameBuffer> for ColorImage {
    fn from(fb: FrameBuffer) -> Self {
        ColorImage::from_rgb([fb.width as usize, fb.height as usize], fb.data.as_slice())
    }
}