extern crate dotext;
mod processor;

use std::io::{Write, Read};
use image::{GenericImageView, ImageBuffer, RgbImage, Rgb};
use ascii_converter::{string_to_decimals, decimals_to_string};
use dotext::*;

use processor::{string_to_image};

fn main() {
  // let mut file = Docx::open("samples/sample.docx").expect("Cannot open file!");
  // let mut string = String::new();
  // let _ = file.read_to_string(&mut string);
  // let processed_isi = string.replace(|c: char| !c.is_ascii(), "");

  // let image = string_to_image(processed_isi);

  // image.save("results/result.png").unwrap();

  // -------------

  let image = image::open("results/result.png").expect("File not found!").to_rgb8();

  let mut raw_bytes: Vec<u8> = image.into_raw();

  raw_bytes.retain(|&c| c >= 32);

  let output = decimals_to_string(&raw_bytes).unwrap();
  println!("Result: {:#?}", output);
  
  // -------------
}
