use getopts::Options;

use std::env;
use std::path::Path;

mod file;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "list of all available arguments");
    opts.optflag("l", "list", "list video files in the current directory and subdirectories");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("l") {
        let video_files = file::get_video_files(Path::new("."));
        for file in video_files {
            println!("{:?}", file);
        }
    }
}
