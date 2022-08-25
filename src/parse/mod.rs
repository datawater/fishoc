#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs::{File};
use std::io::{prelude::*, BufReader, Write};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum OpType {
    Heading_I(String),
    Heading_II(String),
    Heading_III(String),
    Heading_IIII(String),
    Heading_IIIII(String),
    Paragraph(String),
    Link(String, String),
    LineBr,
    ListStart,
    ListEnd,
    ListItemStart,
    ListItemEnd,
    Null
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Location {
    filePath: String,
    line:       u64
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Op {
    Type:       OpType,
    location: Location
}

impl Op {
    pub fn new(t: OpType, l: Location) -> Op {
        return Op {Type: t, location: l};
    }
}

impl Location {
    pub fn new(f: String, l: u64) -> Location {
        return Location {filePath: f, line: l};
    }
}

pub fn parseTokenAsOp<'a>(mut word : &str, mut arg : &str, loc: Location) -> Op {
    word = word.trim(); arg = arg.trim_start();
    if word == "-"        {return Op::new(OpType::Heading_I(arg.to_owned()), loc);}
    if word == "--"       {return Op::new(OpType::Heading_II(arg.to_owned()), loc);}
    if word == "---"      {return Op::new(OpType::Heading_III(arg.to_owned()), loc);}
    if word == "----"     {return Op::new(OpType::Heading_IIII(arg.to_owned()), loc);}
    if word == "-----"    {return Op::new(OpType::Heading_IIIII(arg.to_owned()), loc);}
    if word == "~"        {return Op::new(OpType::Paragraph(arg.to_owned()), loc);}
    if word == "___"     {return Op::new(OpType::LineBr, loc);}
    if word == "="        {return Op::new(OpType::ListStart, loc);}
    if word == "=="      {return Op::new(OpType::ListEnd, loc);}
    if word == "."         {return Op::new(OpType::ListItemStart, loc);}
    if word == ".."        {return Op::new(OpType::ListItemEnd, loc);}
    if word.chars().nth(0) == Some('{') && word.chars().nth(word.len()-1) == Some('}') {
        let mut a : String = "".to_owned();
        for i in 1..word.len() - 1 {a.push_str(&(word.chars().nth(i).expect("ERROR").to_string()));}
        return Op::new(OpType::Link(a.to_owned(), arg.to_owned()), loc);
    }
    return Op::new(OpType::Null, loc);
}

pub fn isToken(mut word : &str) -> bool {
    word = word.trim();
    if word == "-"        {return true;}
    if word == "--"       {return true;}
    if word == "---"      {return true;}
    if word == "----"     {return true;}
    if word == "-----"    {return true;}
    if word == "~"        {return true;}
    if word == "___"     {return true;}
    if word == "="        {return true;}
    if word == "=="      {return true;}
    if word == "."         {return true;}
    if word == ".."        {return true;}
    if word.chars().nth(0) == Some('{') && word.chars().nth(word.len()-1) == Some('}') {return true;}
    return false;
}

pub fn compileOp(op: Op, mut file: File) {
    match op.Type {
       OpType::Heading_I(t) =>          {file.write_all(&format!("<h1>{}</h1>", t).bytes().collect::<Vec<u8>>()).expect("COULDN'T WRITE TO FILE");}
       OpType::Heading_II(t) =>         {file.write_all(&format!("<h2>{}</h2>", t).bytes().collect::<Vec<u8>>()).expect("COULDN'T WRITE TO FILE");}
       OpType::Heading_III(t) =>        {file.write_all(&format!("<h3>{}</h3>", t).bytes().collect::<Vec<u8>>()).expect("COULDN'T WRITE TO FILE");}
       OpType::Heading_IIII(t) =>       {file.write_all(&format!("<h4>{}</h4>", t).bytes().collect::<Vec<u8>>()).expect("COULDN'T WRITE TO FILE");}
       OpType::Heading_IIIII(t) =>     {file.write_all(&format!("<h5>{}</h5>", t).bytes().collect::<Vec<u8>>()).expect("COULDN'T WRITE TO FILE");}
       OpType::Paragraph(t) =>          {file.write_all(&format!("<p>{}</p>", t).bytes().collect::<Vec<u8>>()).expect("COULDN'T WRITE TO FILE");}
       OpType::Link(l, t)  =>                  {file.write_all(&format!("<a href='{}' target='_blank'>{}</a>", t, l).bytes().collect::<Vec<u8>>()).expect("COULDN'T WRITE TO FILE");}
       OpType::LineBr =>                      {file.write_all(b"<br>").expect("COULDN'T WRITE TO FILE");}
       OpType::ListStart =>                  {file.write_all(b"<ul>").expect("COULDN'T WRITE TO FILE");}
       OpType::ListItemStart =>         {file.write_all(b"<li>").expect("COULDN'T WRITE TO FILE");}
       OpType::ListItemEnd =>           {file.write_all(b"</li>").expect("COULDN'T WRITE TO FILE");}
       OpType::ListEnd =>                    {file.write_all(b"</ul>").expect("COULDN'T WRITE TO FILE");}
       OpType::Null =>                          {panic!("Null supplied to compileOp");}
       // _ => Should be unreachable 
    }
}


pub fn compileFile(ifp: File, mut ofp: File, inputFileName: &str) {
    let reader = BufReader::new(ifp);
    let mut line_c: u64 = 0;
    let mut ops: Vec<Op> = vec![]; 

    ofp.write_all(&format!("<html><head><title>{}</title></head<body>", inputFileName).bytes().collect::<Vec<u8>>()).expect("COULDN'T WRITE TO FILE");

    for line in reader.lines() {
        match line {
            Ok(_) => {
                line_c += 1;
                let line = line.unwrap();
                let mut token_vec: Vec<bool> = vec![];
                let line_vec = line.split_whitespace().collect::<Vec<&str>>(); 
                for word in &line_vec.clone() {token_vec.push(isToken(word));}
                for mut i in 0..token_vec.len() {
                    let mut args_ : String = "".to_owned();
                    let p = i.clone();
                    if token_vec[i] == true {
                        i += 1;
                        if i == token_vec.len() {ops.push(parseTokenAsOp(line_vec[p], &args_, Location::new(inputFileName.to_owned(), line_c)));break;}
                        while token_vec[i] != true {
                            args_ += &(" ".to_owned() + line_vec[i]);
                            if i+1 == token_vec.len() {break;}
                            i += 1;
                        }
                        ops.push(parseTokenAsOp(line_vec[p], &args_, Location::new(inputFileName.to_owned(), line_c)));
                    }
                }
            }
            Err(s) => {println!("[ERROR] Couldn't read file! err: {}", s); std::process::exit(1);}
        }
    }

    for op in ops {
        if op.Type == OpType::Null {println!("Error, invalid token in file {} on line {}!", op.location.filePath, op.location.line); std::process::exit(1);}
        compileOp(op, ofp.try_clone().unwrap());
    }

    ofp.write_all(b"</body></html>").expect("COULDN'T WRITE TO FILE");
}
