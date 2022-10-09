use std::env;
use std::process::Command;

fn main() {
    // It is needed to indicate the variable type when it is a collection
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let video = Video::new(&args);
    let outuput = format!("{}(2).mp4", video.file_name);

    let mut compress = Command::new("ffmpeg");
    compress.arg("-i").arg(video.file_path).arg(outuput);
    compress.status().expect("Unable to compress the video");
}

struct Video {
    file_path: String,
    file_name: String,
}

impl Video {
    fn new(args: &[String]) -> Video {
        if args.len() == 1 {
            panic!("Not enough arguments");
        } else if args.len() > 2 {
            panic!("Too many arguments");
        }

        Video {
            file_path: args[1].clone(),
            file_name: name(args),
        }
    }
}

fn name(args: &[String]) -> String {
    let mut name: String = String::new();

    /*
     * When it founds a dot it stops scanning the String.
     * There is possible that the file contains dots,
     * in this case the program will not work properly.
     * I will fix this issue later.
    */
    for letter in args[1].chars() {
        if letter.eq(&'.') { break; }
        name += &*format!("{}", letter);
    }

    name
}