use std::env;
use std::process;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        eprintln!("Expected only 1 name provided");
        process::exit(1);
    }
    println!("Hello {:?}!", args[1])
}