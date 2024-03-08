use getopts::Options;
use std::{env, process};

pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn parse_args() -> (String, Options, getopts::Matches) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "list of all available arguments");
    opts.optflag("l", "list", "list video files in the current directory and subdirectories");
    opts.optopt("p", "path", "set the path to search for video files, default Current directory", "PATH");
    opts.optflag("t", "total", "show total duration of all video files");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, opts);
            process::exit(1);
        }
    };

    (program, opts, matches)
}