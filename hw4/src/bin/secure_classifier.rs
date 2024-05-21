use std::env;
use darknet::image_classifier;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input_filename = &args[1];
  let filename_cstr = std::ffi::CString::new(input_filename.clone()).unwrap();

  let top_k = 3;

  unsafe {
    image_classifier(filename_cstr.as_ptr(), top_k);
  }
}
