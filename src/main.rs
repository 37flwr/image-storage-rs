use std::io::Write;
use ascii_converter::{string_to_decimals, decimals_to_string};

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

    let image = decimals_to_image(decimals);

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
