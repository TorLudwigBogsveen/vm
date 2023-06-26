/*
 *   Copyright (c) 2022 Ludwig Bogsveen
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

pub enum OutputDirective {
    U8,
    S8,
    U16,
    S16,
    H8,
    H16,
    B8,
    B16,
    C8,
    C16,
}

impl From<u8> for OutputDirective {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::U8,
            1 => Self::S8,
            2 => Self::U16,
            3 => Self::S16,
            4 => Self::H8,
            5 => Self::H16,
            6 => Self::B8,
            7 => Self::B16,
            8 => Self::C8,
            9 => Self::C16,
            _ => panic!(),
        }
    }
}

impl From<OutputDirective> for u8 {
    fn from(val: OutputDirective) -> Self {
        match val {
            OutputDirective::U8     => 0,
            OutputDirective::S8     => 1,
            OutputDirective::U16    => 2,
            OutputDirective::S16    => 3,
            OutputDirective::H8     => 4,
            OutputDirective::H16    => 5,
            OutputDirective::B8     => 6,
            OutputDirective::B16    => 7,
            OutputDirective::C8     => 8,
            OutputDirective::C16    => 9,
        }
    }
}