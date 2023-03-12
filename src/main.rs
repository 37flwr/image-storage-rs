use std::io::Write;
use image::{GenericImageView, ImageBuffer, RgbImage, Rgb};
use ascii_converter::{binary_to_decimal, string_to_binary, string_to_decimals, decimals_to_string, decimals_to_binary, binary_to_string};

mod processor;
use processor::{decimals_to_image};

fn read_string() -> String {
  let mut text = String::new();
  std::io::stdin().read_line(&mut text)
    .expect("Error while getting input");
  return text;
}

fn main() {
    println!("Type:...");
    let input = read_string();

    let mut decimals = string_to_decimals(&input).unwrap();

    let image = decimals_to_image(decimals);

    image.save("test.png").unwrap();

    // -------------

    // let image_rgb8 = image::open("test.png").expect("File not found!").to_rgb8();

    // let mut raw_bytes: Vec<u8> = image_rgb8.into_raw();
    
    // while *raw_bytes_test.last().unwrap() == 0 {
    //     raw_bytes_test.pop();
    // }

    // let output = decimals_to_string(&raw_bytes_test).unwrap();
    // println!("Result: {:#?}", output);
    
    // -------------
}
