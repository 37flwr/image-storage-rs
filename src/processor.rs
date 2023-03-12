use image::{GenericImageView, ImageBuffer, RgbImage, Rgb};

fn chunks_to_rgb_pixels(total_pixels: u32, sqr_side: u32, chunks: Vec<&[u8]>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut output = RgbImage::new(sqr_side, sqr_side);

    let mut x_axis: u32 = 0;
    let mut y_axis: u32 = 0;
    

    for chunk in 0..chunks.len() {
        output.put_pixel(x_axis, y_axis, 
            Rgb::from([chunks[chunk][0], chunks[chunk][1], chunks[chunk][2]])
        );
        if x_axis == sqr_side - 1 {
            x_axis = 0;
            y_axis+=1
        } else {
            x_axis+=1;
        }
    }

    output
}

pub fn decimals_to_image(decimals: Vec<u8>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let total_pixels: u32 = (decimals.len() as f64 / 3f64).ceil() as u32;
    let sqr_side: u32 = (decimals.len() as f64 / 3f64).sqrt().ceil() as u32;

    let mut chunks: Vec<&[u8]> = decimals.chunks(3).collect();

    let mut new_last_chunk: Vec<u8> = vec![];

    if chunks.last().unwrap().len() < 3 {
        for val in chunks.last().unwrap().iter() {
            new_last_chunk.push(*val);
        }
        while new_last_chunk.len() < 3 {
            new_last_chunk.push(0);
        }

        chunks.pop();
        chunks.push(new_last_chunk.as_slice());
    }

    chunks_to_rgb_pixels(total_pixels, sqr_side, chunks)
}