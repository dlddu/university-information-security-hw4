use darknet::image_classifier;
use std::{env, ffi::CString, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let parameters = fs::read_to_string(input_filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(':')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    for parameter in parameters {
        let _user = parameter.first().unwrap();
        let filename = CString::new(parameter.get(1).unwrap().to_string()).unwrap();
        let top_k = parameter.get(2).unwrap().parse::<i32>().unwrap();

        unsafe { image_classifier(filename.as_ptr(), top_k) };
    }
}
