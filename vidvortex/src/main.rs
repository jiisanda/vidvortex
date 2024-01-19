mod file;

fn main() {
    let video_files = file::get_video_files(std::path::Path::new("."));
    for file in video_files {
        println!("{:?}", file);
    }
}
