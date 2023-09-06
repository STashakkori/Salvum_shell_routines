// Salvum mkdir routine
// Author: Sina Tashakkori, QVLx Labs

use std::fs;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 || args.len() > 2 { return; }

  if args[1].trim().starts_with("-") { return; }

  match fs::create_dir(args[1].trim()) {
    Ok(_) => println!("\x1b[38;5;83mSuccessfully created {} directory.\x1b[0m",args[1].trim()),
    Err(_) => println!("\x1b[38;5;203mParent directories must exist, child to be created must not.\x1b[0m"),
  };
}