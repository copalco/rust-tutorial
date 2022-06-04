// Lesson #61
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let i = 0;
  while i < vec_1.len() {}

  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {

    rgba.push(i as u8);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_rgba_func_returns_start_to_end() {
    let a = set_rgba(vec![0u8, 1u8, 2u8, 3u8, 4u8], 0usize, 3usize);
    assert_eq!(a, vec![0, 1, 2, 3]);
  }
  #[test]
  #[should_panic]
  fn set_rgba_func_panics_on_invalid_index() {
    let a = set_rgba(vec![10u8, 12u8, 32u8, 34u8, 54u8], 0usize, 5usize);
  }
}

