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

use core::panic;
use std::vec::Vec;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::fs::File;
use std::io::prelude::*;

use crate::instructions::*;

lazy_static! {
  static ref REGISTERS_U8: HashMap<String, u8> = {
    let mut map = HashMap::new();
    map.insert(String::from("HA"), 0);
    map.insert(String::from("LA"), 1);
    map.insert(String::from("HB"), 2);
    map.insert(String::from("LB"), 3);
    map.insert(String::from("HC"), 4);
    map.insert(String::from("LC"), 5);
    map.insert(String::from("HD"), 6);
    map.insert(String::from("LD"), 7);
    map
  };

  static ref REGISTERS_U16: HashMap<String, u8> = {
    let mut map = HashMap::new();
    map.insert(String::from("RA"), 0);
    map.insert(String::from("RB"), 1);
    map.insert(String::from("RC"), 2);
    map.insert(String::from("RD"), 3);
    map.insert(String::from("RE"), 4);
    map.insert(String::from("RF"), 5);
    map.insert(String::from("IP"), 6);
    map.insert(String::from("SP"), 7);
    map
  };

  static ref INSTRUCTION_TOKENS: HashMap<String, InstructionToken> = {
    let mut tokens_map = HashMap::new();
    let instruction_tokens = vec![
      ("NOP", InstructionToken::new(Instruction::NOP, &[])),
      ("BRK", InstructionToken::new(Instruction::BRK, &[])),
      ("MOVRR", InstructionToken::new(Instruction::MOVRR, &[
        OperandToken::Reg8,
        OperandToken::Reg8,
      ])),
      ("MOVRM", InstructionToken::new(Instruction::MOVRM, &[
        OperandToken::Reg8,
        OperandToken::Ptr,
      ])),
      ("MOVMR", InstructionToken::new(Instruction::MOVMR, &[
        OperandToken::Ptr,
        OperandToken::Reg8,
      ])),
      ("MOVMM", InstructionToken::new(Instruction::MOVMM, &[
        OperandToken::Ptr,
        OperandToken::Ptr,
      ])),
      ("MOVIR", InstructionToken::new(Instruction::MOVIR, &[
        OperandToken::Val8,
        OperandToken::Reg8,
      ])),
      ("MOVIM", InstructionToken::new(Instruction::MOVIM, &[
        OperandToken::Char,
        OperandToken::Ptr,
      ])),
      ("MOVRP", InstructionToken::new(Instruction::MOVRP, &[
        OperandToken::Reg8,
        OperandToken::Reg16,
      ])),
      ("MOVPR", InstructionToken::new(Instruction::MOVPR, &[
        OperandToken::Reg16,
        OperandToken::Reg8,
      ])),
      ("MOVDRR", InstructionToken::new(Instruction::MOVDRR, &[
        OperandToken::Reg16,
        OperandToken::Reg16,
      ])),
      ("MOVDRM", InstructionToken::new(Instruction::MOVDRM, &[
        OperandToken::Val8,
        OperandToken::Ptr,
      ])),
      ("MOVDMR", InstructionToken::new(Instruction::MOVDMR, &[
        OperandToken::Ptr,
        OperandToken::Reg16,
      ])),
      ("MOVDMM", InstructionToken::new(Instruction::MOVDMM, &[
        OperandToken::Ptr,
        OperandToken::Ptr,
      ])),
      ("MOVDIR", InstructionToken::new(Instruction::MOVDIR, &[
        OperandToken::Ptr,
        OperandToken::Reg16,
      ])),
      ("MOVDIM", InstructionToken::new(Instruction::MOVDIM, &[
        OperandToken::Ptr,
        OperandToken::Ptr,
      ])),
      ("MOVDRP", InstructionToken::new(Instruction::MOVDRP, &[
        OperandToken::Reg16,
        OperandToken::Reg16,
      ])),
      ("MOVDPR", InstructionToken::new(Instruction::MOVDPR, &[
        OperandToken::Reg16,
        OperandToken::Reg16,
      ])),
      ("INC", InstructionToken::new(Instruction::INC, &[
        OperandToken::Reg8,
      ])),
      ("DEC", InstructionToken::new(Instruction::DEC, &[
        OperandToken::Reg8,
      ])),
      ("INCD", InstructionToken::new(Instruction::INCD, &[
        OperandToken::Reg16,
      ])),
      ("DECD", InstructionToken::new(Instruction::DECD, &[
        OperandToken::Reg16,
      ])),
      ("CMP", InstructionToken::new(Instruction::CMP, &[
        OperandToken::Reg8,
        OperandToken::Reg8,
      ])),
      ("CMPD", InstructionToken::new(Instruction::CMPD, &[
        OperandToken::Reg16,
        OperandToken::Reg16,
      ])),
      ("JMP", InstructionToken::new(Instruction::JMP, &[
        OperandToken::Reg16,
      ])),
      ("JNE", InstructionToken::new(Instruction::JNE, &[
        OperandToken::Reg16,
      ])),
      ("JE", InstructionToken::new(Instruction::JE, &[
        OperandToken::Reg16,
      ])),
      ("JG", InstructionToken::new(Instruction::JG, &[
        OperandToken::Reg16,
      ])),
      ("JL", InstructionToken::new(Instruction::JL, &[
        OperandToken::Reg16,
      ])),
      ("JC", InstructionToken::new(Instruction::JC, &[
        OperandToken::Reg16,
      ])),
      ("PUSH", InstructionToken::new(Instruction::PUSH, &[
        OperandToken::Reg8,
      ])),
      ("POP", InstructionToken::new(Instruction::POP, &[
        OperandToken::Reg8,
      ])),
      ("PUSHD", InstructionToken::new(Instruction::PUSHD, &[
        OperandToken::Reg16,
      ])),
      ("POPD", InstructionToken::new(Instruction::POPD, &[
        OperandToken::Reg16,
      ])),
      ("CALL", InstructionToken::new(Instruction::CALL, &[
        OperandToken::Reg16,
      ])),
      ("CALLI", InstructionToken::new(Instruction::CALLI, &[
        OperandToken::Ptr,
      ])),
      ("RET", InstructionToken::new(Instruction::RET, &[])),
      ("INT", InstructionToken::new(Instruction::INT, &[
        OperandToken::Reg8,
      ])),
    ];
  
    for ins_token in instruction_tokens {
      tokens_map.insert(ins_token.0.to_string(), ins_token.1);
    }

    tokens_map
  };
}

#[derive(Clone)]
enum OperandToken {
  Val8,
  Val16,
  Val32,
  Val64,
  Ptr,
  Char,
  Reg8,
  Reg16,
}

struct InstructionToken {
  instruction: Instruction,
  operands: Vec<OperandToken>,
}

impl InstructionToken {
  fn new(instruction: Instruction, operands: &[OperandToken]) -> InstructionToken {
    InstructionToken {
      instruction,
      operands: operands.to_vec(),
    }
  }
}

fn assemble_instructions(bin: &mut Vec<u8>, tokens: Vec<Token>) {
  let tokens = tokens;
  let lable_tokens_indices = assemble_lables(&tokens);
  let tokens: Vec<Token> = tokens.into_iter().filter(|t|  { match t {
    Token::Operand(_) => true,
    Token::Instruction(_) => true,
    Token::Data(_) => true,
    _ => false,
  }}).collect();


  for lable in &lable_tokens_indices {
    //println!("{} : {}", lable.0, lable.1);
  }

  let mut lable_locations = HashMap::new();
  let mut lable_destinations = Vec::new();
  /*let mut lables = {
    let mut lbls = HashMap::new(); 
    for lable in lables {
      lbls.insert(lable.0, (lable.1, 0));
    } 
    lbls
  };

  let mut lable_locations = Vec::new();*/

  let mut index = 0;
  while index < tokens.len() {
    let mut index_add = |index: &mut usize, bin: &Vec<u8>| {
      for lable in &lable_tokens_indices {
        if *lable.1 == (*index) as u16 {
          //println!("{}", );
          //println!("init {}: {}: {}", *lable.0, index, bin.len() as u16 - 1);
          lable_locations.insert(lable.0 as &str, bin.len() as u16 - 1);
        }
      }
      *index += 1;
    };

    match &tokens[index] {
      Token::Instruction(ins) => {
        //println!("Ins: {} at: {} : {}", ins, index, bin.len());
        match INSTRUCTION_TOKENS.get(ins) {
          Some(ins_token) => {
            bin.push(ins_token.instruction as u8);
            for operand in &ins_token.operands {
              index_add(&mut index, &bin);
              match operand {
                OperandToken::Val8 => {
                  bin.push(tokens[index].as_operand().unwrap().parse().unwrap());
                }
                OperandToken::Val16 => {
                  bin.push(tokens[index].as_operand().unwrap().parse().unwrap());
                  bin.push(0);
                }
                OperandToken::Val32 => {
                  bin.push(tokens[index].as_operand().unwrap().parse().unwrap());
                  bin.push(0);
                  bin.push(0);
                  bin.push(0);
                }
                OperandToken::Val64 => {
                  bin.push(tokens[index].as_operand().unwrap().parse().unwrap());
                  bin.push(0); bin.push(0); bin.push(0);
                  bin.push(0); bin.push(0); bin.push(0); bin.push(0);
                }
                OperandToken::Ptr => {
                  match tokens[index].as_operand().unwrap().parse::<u16>() {
                    Ok(num) => {
                      bin.push(num as u8);
                      bin.push((num >> 8) as u8);
                    }
                    Err(_) => {
                      lable_destinations.push((tokens[index].as_operand().unwrap().clone(), bin.len()));
                      bin.push(0);
                      bin.push(0);
                    }
                  }
                }
                OperandToken::Char => {
                  bin.push(tokens[index].as_operand().unwrap().as_bytes()[0]);
                }
                OperandToken::Reg8 => {
                  match REGISTERS_U8.get(tokens[index].as_operand().unwrap()) {
                    Some(num) => {
                      bin.push(*num);
                    }
                    None => {
                      if let Ok(num) = tokens[index].as_operand().unwrap().parse::<u8>() {
                        bin.push(num);
                      } else {
                        println!("ERROR TOKEN IS NOT A VALID 8 BIT REGISTER!!!", /*tokens[index].as_operand().unwrap()*/);
                      }
                    }
                  }
                }
                OperandToken::Reg16 => {
                  match REGISTERS_U16.get(tokens[index].as_operand().unwrap()) {
                    Some(num) => {
                      bin.push(*num);
                    }
                    None => {
                      if let Ok(num) = tokens[index].as_operand().unwrap().parse::<u8>() {
                        bin.push(num);
                      } else {
                        println!("ERROR TOKEN IS NOT A VALID 16 BIT REGISTER!!!", /*tokens[index].as_operand().unwrap()*/);
                      }
                    }
                  }
                }
              }
            }
          }
          None => {println!("Not a valid token", /*token*/)}
        }
        index_add(&mut index, &bin);
      }
      Token::Data(d) => {
        match d as &str {
          "byte" => {
            bin.push(tokens[index+1].as_operand().unwrap().parse::<u8>().unwrap());
            //println!("{}", bin[bin.len()-1]);
          }
          "char" => {
            bin.push(tokens[index+1].as_operand().unwrap().as_bytes()[0]);
            //println!("{}", bin[bin.len()-1]);
          }
          "str" => {
            bin.extend_from_slice(tokens[index+1].as_operand().unwrap().as_bytes());
          }
          _ => panic!()
        }
        index_add(&mut index, &bin);
        index_add(&mut index, &bin);
      }
      _=> { panic!(); }
    } 
  }

  for lable in lable_destinations {
    //println!("[{}][{}]", lable.0, lables[&lable.0].1);
    bin[lable.1]      = lable_locations[lable.0] as u8;
    bin[lable.1 + 1]  = (lable_locations[lable.0] >> 8) as u8;
  }
}

fn assemble_lables(tokens: &[Token]) -> HashMap<String, u16> {
  let mut lables = HashMap::new();
  let mut index = 0;
  for token in tokens {
    match token {
      Token::Instruction(_) => {
        index += 1;
      }
      Token::Operand(_) => {
        index += 1;
      }
      Token::Data(_) => {
        index += 1;
      }
      Token::Lable(l) => {
        lables.insert(l.clone(), index);
      }
      _=> {}
    }
    //index += 1;
  }

  lables
}

/*fn assemble_macros(tokens: &mut Vec<Token>) {
  let mut token_iter = tokens.iter();
  while let Some(token) = token_iter.next() {
    match token {
      Token::Macro(m) => {
        let m: &str = &m.to_uppercase();
        match m {
          "INCLUDE" => {
            match token_iter.next().unwrap() {
              Token::Operand(file_name) => {
                assemble_file(file_name);
                //tokens.
              }
              _ => {
                println!("chrash i assemble macros prut.");
              }
            }
          }
          _=> {}
        }
      }
      _=> {}
    }
  }
}*/

fn assemble_file(file_name: &str) -> Vec<Token> {
  let mut file = File::open(file_name).unwrap();
  let mut file_contents = String::new();
  file.read_to_string(&mut file_contents).unwrap();

  let words: Vec<&str> = file_contents.split_ascii_whitespace().collect();
  let mut tokens = Vec::new();

  let mut index = 0;
  while index < words.len() {
    let token = Token::new(&words[index..], &mut index);
    tokens.push(token);
    index += 1;
  }

  tokens
}

enum Token {
  None,
  Comment(String),
  Instruction(String),
  Lable(String),
  Operand(String),
  Macro(String),
  Data(String),
}

impl Token {
  /*pub fn data_to_bytes(&self) -> Vec<u8> {
    match self {
      Token::Data(d) => {
        let words = d.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut data = Vec::new();
        match words[0] {
          "byte" => {data.push(words[1].parse().unwrap())}
          "str" => {data.extend_from_slice(words[1].as_bytes())}
          _ => panic!()
        }

        return data;
      }
      _=> panic!() 
    }
  }*/

  pub fn new(raw_data: &[&str], index: &mut usize) -> Token {
    let chars: Vec<char> = raw_data[0].chars().collect();

    if chars[0] == ':' {
      let mut lable = String::from_iter(chars.iter());
      lable.remove(0);
      return Token::Lable(lable);
    }

    if raw_data[0].starts_with("/*") {

      let mut comment = String::new();

      for i in 0..raw_data.len() {
        if raw_data[i].ends_with("*/") {
          comment.push_str(raw_data[i].split("*/").collect::<Vec<&str>>()[0]);
          comment = String::from(comment.trim());
          *index += i;
          break;
        }

        comment.push_str(raw_data[i]);
        comment.push(' ');
      }

      comment.remove(0);
      comment.remove(0);

      return Token::Comment(comment);
    }

    if raw_data[0].starts_with("#") {
      let mut macro_name = String::from_iter(chars.iter());
      macro_name.remove(0);
      return Token::Macro(macro_name);
    }

    if raw_data[0].starts_with(".") {
      let mut data = String::from_iter(chars.iter());
      data.remove(0);
      return Token::Data(data);
    }

    match INSTRUCTION_TOKENS.get(raw_data[0]) {
      Some(_) => {
        return Token::Instruction(String::from(raw_data[0]));
      }
      None => {}
    }

    Token::Operand(String::from(raw_data[0]))
  }

  fn as_instruction(&self) -> Option<&str> { match self { Token::Instruction(ins) => Some(ins), _=> None } }
  fn as_operand    (&self) -> Option<&str> { match self { Token::Operand(op)      => Some(op),  _=> None } }
} 

pub fn assemble(file_name: &str) -> Vec<u8> {
  let mut bin = Vec::new();

  let tokens = assemble_file(file_name);
  let mut index = 0;
  /*for (_index, token) in tokens.iter().enumerate() {
    print!("Index: {} ", index);
    match token {
      Token::Lable(l) => {
        println!("Lable: {}", l);
      }
      Token::Comment(c) => {
        println!("Comment: {}", c);
      }
      Token::Instruction(i) => {
        index += 1;
        println!("Instruction: {}", i);
      }
      Token::Operand(i) => {
        index += 1;
        println!("Operand: {}", i);
      }
      Token::Macro(m) => {
        println!("Macro: {}", m);
      }
      Token::Data(d) => {
        println!("Data: {}", d);
      }
      _ => {
        println!("NONE!");
      }
    }
  }*/


 
  assemble_instructions(&mut bin, tokens);  

  //let mut tokens = assemble_labels(&words);
  //assemble_instructions(&mut bin, &tokens.0, &tokens.1);

  bin
}