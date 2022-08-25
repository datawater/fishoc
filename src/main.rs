mod parse;
use std::env;
use std::fs::{File, OpenOptions, remove_file};

fn usage() {
    println!("author: datawater <datawater1@gmail.com>");
    println!("version: 0.1");
    println!("Copyright datawater 2022");
    println!("Usage:\n\t-h or -help\n\t\tDisplays this help message");
}

fn main() {
    let mut input: &str = "";

    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        if args[i] == "-help" || args[i] == "-h" {
            usage();
            std::process::exit(0);
        }
        if args[i].chars().nth(0) == Some('-') {
            println!("Invalid option `{}`. Use `fishoc -help` for help.", args[i]);
            std::process::exit(1);
        }
        if input.len() > 0 {
            println!(
                "Already supplied input file. use `fishoc -help` for help. (error on argument {})",
                i
            );
            std::process::exit(1);
        }
        input = &args[i];
    }
    let output =  input.split(".").collect::<Vec<&str>>()[0].to_owned() + ".html";

    match remove_file(output.clone()) {Err(_) => {} _ => {}}

    let ifp = File::open(input); let ofp = OpenOptions::new().create(true).write(true).open(output.clone());
    match ifp {Err(ref s) =>  {println!("Couldn't open file {}, error: {}", input, s); std::process::exit(1);} Ok(_) => {}}
    match ofp {Err(ref s) => {println!("Couldn't open file {}, error: {}", output.clone(), s); std::process::exit(1);} Ok(_) => {}}
    parse::compileFile(ifp.unwrap(), ofp.unwrap(), input);
}
