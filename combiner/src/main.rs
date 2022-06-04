// Lesson #51
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

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn floating_image_struct_width() {
    let float = FloatingImage::new(1u32, 2u32, "test".to_string());
    assert_eq!(float.width, 1);
  }
  #[test]
  fn floating_image_struct_height() {
    let float = FloatingImage::new(0u32, 10u32, "test".to_string());
    assert_eq!(float.height, 10);
  }
  #[test]
  fn floating_image_struct_name() {
    let float = FloatingImage::new(0u32, 0u32, "output".to_string());
    assert_eq!(float.name, String::from("output"));
  }
  #[test]
  fn floating_image_struct_data() {
    let float = FloatingImage::new(0u32, 0u32, "test".to_string());
    assert_eq!(float.data.capacity(), 3_655_744);
  }
  #[test]
  fn output_var_declared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"let\s+output", file_contents));
      assert!(reg_with_con(
        r"FloatingImage::new\(\s*image_1\.width\(\)\s*,\s*image_1\.height\(\)\s*,\s*args\.output",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}

