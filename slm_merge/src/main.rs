// Salvum merge directories routine
// Author: Sina Tashakkori, QVLx Labs

extern crate fstream;
extern crate walkdir;

use std::path::Path;
use walkdir::WalkDir;
use std::env;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::copy;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args[1].eq("__test__") {
    println!("test successful");
    return;
  }

  if args.len() != 5 { return; }

  let dir1 = args[1].trim();
  let dir2 = args[2].trim();
  let extension = args[3].trim().replace(".","");
  let output = args[4].trim();

  let mut regexp = false;
  merge_dir(&dir1, &dir2, &extension, &output);
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
  Path::new(filename)
    .extension()
    .and_then(OsStr::to_str)
}

fn merge_dir(one_dir: &str, two_dir: &str, extension: &str, output_dir: &str) {
  let one_path = Path::new(one_dir);

  create_dir_all(output_dir);

  for file in WalkDir::new(one_path).into_iter().filter_map(|file| file.ok()) {
    if file.metadata().unwrap().is_file() {
      if get_extension_from_filename(file.file_name().to_str().unwrap()) != Some(extension) { continue; } 

      let mut dest_path = String::from(output_dir);
      dest_path.push_str("/");
      dest_path.push_str(file.file_name().to_str().unwrap());

      copy(file.path(),dest_path);
    }
  }

  let two_path = Path::new(two_dir);

  for file in WalkDir::new(two_path).into_iter().filter_map(|file| file.ok()) {
    if file.metadata().unwrap().is_file() {
      if get_extension_from_filename(file.file_name().to_str().unwrap()) != Some(extension) { continue; } 

      let mut dest_path = String::from(output_dir);
      dest_path.push_str("/");
      dest_path.push_str(file.file_name().to_str().unwrap());

      copy(file.path(),dest_path);
    }
  }
}