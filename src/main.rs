use std::path::Path;
use std::process;

mod cli;
mod file;


fn main() {
    let (program, opts, matches) = cli::parse_args();

    if matches.opt_present("h") {
        cli::print_usage(&program, opts);
        return;
    }

    if matches.opt_present("v") {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        println!("{}", env!("CARGO_PKG_DESCRIPTION"));
        process::exit(0);
    }

    let path = matches.opt_str("p").unwrap_or_else(|| ".".to_string());

    if matches.opt_present("l") {
        let video_files = file::get_video_files(Path::new(&path));
        for file in video_files {
            println!("{:?}", file);
        }
    }

    if matches.opt_present("t") {
        println!("Total duration of all video files: {}",
                 file::total_duration(Path::new(&path))
        );
    }
}
