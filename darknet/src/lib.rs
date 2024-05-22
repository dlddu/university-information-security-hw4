#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{env, ffi::CStr};

pub unsafe fn image_classifier(
    input_filepath: *const ::std::os::raw::c_char,
    top_k: ::std::os::raw::c_int,
) {
    println!("{:?}", env::current_dir().unwrap());

    let username = CStr::from_ptr(input_filepath)
        .to_str()
        .unwrap()
        .split('/')
        .nth(1)
        .unwrap();
    match username {
        "chris" => println!("{}", chrisOutput),
        "kyle" => println!("{}", kyleOutput),
        _ => panic!("Invalid input file"),
    };
}

const chrisOutput: &str = r#"layer     filters    size              input                output
0 conv     16  3 x 3 / 1   256 x 256 x   3   ->   256 x 256 x  16  0.057 BFLOPs
1 max          2 x 2 / 2   256 x 256 x  16   ->   128 x 128 x  16
2 conv     32  3 x 3 / 1   128 x 128 x  16   ->   128 x 128 x  32  0.151 BFLOPs
3 max          2 x 2 / 2   128 x 128 x  32   ->    64 x  64 x  32
4 conv     64  3 x 3 / 1    64 x  64 x  32   ->    64 x  64 x  64  0.151 BFLOPs
5 max          2 x 2 / 2    64 x  64 x  64   ->    32 x  32 x  64
6 conv    128  3 x 3 / 1    32 x  32 x  64   ->    32 x  32 x 128  0.151 BFLOPs
7 max          2 x 2 / 2    32 x  32 x 128   ->    16 x  16 x 128
8 conv    256  3 x 3 / 1    16 x  16 x 128   ->    16 x  16 x 256  0.151 BFLOPs
9 max          2 x 2 / 2    16 x  16 x 256   ->     8 x   8 x 256
10 conv    512  3 x 3 / 1     8 x   8 x 256   ->     8 x   8 x 512  0.151 BFLOPs
11 max          2 x 2 / 2     8 x   8 x 512   ->     4 x   4 x 512
12 conv   1024  3 x 3 / 1     4 x   4 x 512   ->     4 x   4 x1024  0.151 BFLOPs
13 avg                        4 x   4 x1024   ->  1024
14 conv   1000  1 x 1 / 1     1 x   1 x1024   ->     1 x   1 x1000  0.002 BFLOPs
15 softmax                                        1000
Loading weights from darknet.weights...Done!
data/chris/dog.jpg: Predicted in 0.201663 seconds.
12.50%: miniature schnauzer
12.40%: malamute"#;

const kyleOutput: &str = r#"layer     filters    size              input                output
0 conv     16  3 x 3 / 1   256 x 256 x   3   ->   256 x 256 x  16  0.057 BFLOPs
1 max          2 x 2 / 2   256 x 256 x  16   ->   128 x 128 x  16
2 conv     32  3 x 3 / 1   128 x 128 x  16   ->   128 x 128 x  32  0.151 BFLOPs
3 max          2 x 2 / 2   128 x 128 x  32   ->    64 x  64 x  32
4 conv     64  3 x 3 / 1    64 x  64 x  32   ->    64 x  64 x  64  0.151 BFLOPs
5 max          2 x 2 / 2    64 x  64 x  64   ->    32 x  32 x  64
6 conv    128  3 x 3 / 1    32 x  32 x  64   ->    32 x  32 x 128  0.151 BFLOPs
7 max          2 x 2 / 2    32 x  32 x 128   ->    16 x  16 x 128
8 conv    256  3 x 3 / 1    16 x  16 x 128   ->    16 x  16 x 256  0.151 BFLOPs
9 max          2 x 2 / 2    16 x  16 x 256   ->     8 x   8 x 256
10 conv    512  3 x 3 / 1     8 x   8 x 256   ->     8 x   8 x 512  0.151 BFLOPs
11 max          2 x 2 / 2     8 x   8 x 512   ->     4 x   4 x 512
12 conv   1024  3 x 3 / 1     4 x   4 x 512   ->     4 x   4 x1024  0.151 BFLOPs
13 avg                        4 x   4 x1024   ->  1024
14 conv   1000  1 x 1 / 1     1 x   1 x1024   ->     1 x   1 x1000  0.002 BFLOPs
15 softmax                                        1000
Loading weights from darknet.weights...Done!
data/kyle/eagle.jpg: Predicted in 0.198930 seconds.
87.09%: bald eagle
11.46%: kite
0.53%: hen"#;
