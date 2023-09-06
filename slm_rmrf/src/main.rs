// Salvum rm -rf routine
// Author: Sina Tashakkori, QVLx Labs

use rm_rf;
use std::env;
use std::fs::metadata;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 { return; }

  let md = match metadata(args[1].trim()) {
    Ok(md) => md,
    Err(_) => return,
  };

  if !md.is_dir() { return; }

  if args[1].trim().eq("/") { return; }

  let file_vec: Vec<&str> = args[1].trim().split("/").collect();

  let root_dir = file_vec[1];

  let nono_direc = vec!["boot", "dev", "etc", "root", "sys" , "usr"];

  for dir in nono_direc { if root_dir.eq(dir) { return; } }

  match rm_rf::ensure_removed(args[1].trim()){
    Ok(_) => (),
    Err(_) => (),
  };
}
