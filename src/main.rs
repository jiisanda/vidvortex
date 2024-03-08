use std::path::Path;

mod cli;
mod file;


fn main() {
    let (program, opts, matches) = cli::parse_args();

    if matches.opt_present("h") {
        cli::print_usage(&program, opts);
        return;
    }

    let path = matches.opt_str("p").unwrap_or_else(|| ".".to_string());

    if matches.opt_present("l") {
        let video_files = file::get_video_files(Path::new(&path));
        for file in video_files {
            println!("{:?}", file);
        }
    }
}
