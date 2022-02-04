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

 pub struct RAM {
    pub buff: [u8; 65536],
  }
  
  impl RAM {
    pub fn new() -> RAM {
      RAM {
        buff: [0; 65536]
      }
    }
  
    pub fn get(&self, index: u16) -> u8 {
      let val = self.buff[index as usize];
      //println!("memget: {:X} : {:X}", index, val);
      val
    }
  
    pub fn get_ptr(&self, index: u16) -> u16 {
      self.get(index) as u16 + ((self.get(index + 1) as u16) << 8)
    }
  
    pub fn set(&mut self, index: u16, val: u8) {
      //println!("DST: {} SRC: {}", index, val);
      self.buff[index as usize] = val;
      //println!("memset: {:X} : {:X}", index, val);
    }
  
    pub fn set_multiple(&mut self, index: u16, vals: &[u8]) {
      let mut index = index;
      for val in vals {
        self.set(index, *val);
        index += 1;
      }
    }
  
    pub fn set_ptr(&mut self, index: u16, val: u16) {
      self.set(index+1, (val >> 8) as u8); self.set(index, val as u8);
    }
  }