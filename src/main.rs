use image::{GenericImageView, ImageBuffer, RgbImage, Rgb};
use std::io::Write;
use ascii_converter::{string_to_decimals, decimals_to_string};
use std::num;
use std::iter::FromIterator;

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
    decimals.pop();

    println!("Length: {}", decimals.len());

    let sqr_side = (decimals.len() as f64 / 3f64).sqrt().ceil() as u32;
    println!("Minimal size: {}", sqr_side);

    let image = decimals_to_image(decimals);
    println!("chunks: {:#?}", image);

    image.save("test.png").unwrap();

    // let striiing = decimals_to_string(&decimals).unwrap();

    // println!("Decimals: {:?}, String: {:?}", decimals, striiing)

    // let img = image::open("1.png").expect("File not found!");
    // println!("{:#?}", img.dimensions());
    // let (width, height) = img.dimentions();
    // for y in 0..height {
    //     for x in 0..width {

    //     }
    // }
    // println!("{:?}", img.pixels());
    // println!("{:?}", img.get_pixel(1, 0));
    // for pixel in img.pixels() {
    //     println!("{:#?}", pixel);
    //     // modify RGBA pixel
    // }
}
