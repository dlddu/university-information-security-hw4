use darknet::image_classifier;
use std::{env, ffi::CString, fs, path::Path};

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
        let user = parameter.first().unwrap();
        let filename = parameter.get(1).unwrap();
        let filepath = CString::new(
            Path::new("data")
                .join(user)
                .join(filename)
                .to_str()
                .unwrap(),
        )
        .unwrap();
        let top_k = parameter.get(2).unwrap().parse::<i32>().unwrap();

        unsafe { image_classifier(filepath.as_ptr(), top_k) };
    }
}
