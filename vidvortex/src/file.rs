use std::path::Path;
use walkdir::WalkDir;
use mime_guess::from_path;


pub fn get_video_files(dir: &Path) -> Vec<String> {
    let vec = WalkDir::new(dir)
        .into_iter()                    // converts walkdir instance to iterator
        .filter_map(Result::ok)           // filter: entries that could be read without errors
        .filter(|e| !e.file_type().is_dir())        // filter: includes entries that are not directory
        .filter(|e| {                       // filter: entries that are video files checked using mime
            let mime = from_path(e.path()).first_raw().unwrap_or("");
            mime.starts_with("video/")
        })
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect();
    vec
}
