use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str;
use walkdir::WalkDir;
use mime_guess::from_path;

pub fn get_video_length(file_path: &Path) -> Result<(f64, String), Box<dyn Error>> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(file_path.to_str().ok_or("Failed to convert path to string")?)
        .output()?;

    let output_str = str::from_utf8(&output.stderr);

    let duration_line = output_str.unwrap().lines()
        .find(|line| line.contains("Duration"))
        .ok_or("Failed to find duration in output")?;

    let duration_str: String = duration_line.split("Duration: ").nth(1)
        .and_then(|s| s.split(',').next())
        .ok_or("Failed to parse duration from output")?
        .to_string();

    let time_parts: Vec<&str> = duration_str.split(":").collect();
    let hrs: f64 = time_parts[0].parse()?;
    let min: f64 = time_parts[1].parse()?;
    let sec: f64 = time_parts[2].parse()?;

    let duration = hrs * 3600.0 + min * 60.0 + sec;

    Ok((duration, duration_str))
}

pub fn get_video_files(dir: &Path) -> Vec<(String, String, String)> {
    let vec = WalkDir::new(dir)
        .into_iter()                    // converts walkdir instance to iterator
        .filter_map(Result::ok)           // filter: entries that could be read without errors
        .filter(|e| !e.file_type().is_dir())        // filter: includes entries that are not directory
        .filter(|e| {                       // filter: entries that are video files checked using mime
            let mime = from_path(e.path()).first_raw().unwrap_or("");
            mime.starts_with("video/")
        })
        .filter_map(|e| {
            let path = e.path();
            let metadata = fs::metadata(path).ok()?;
            let size_mb = metadata.len() as f64 / 1_000_000.0;
            let (_, duration) = get_video_length(path).ok()?;
            Some((path.to_string_lossy().into_owned(), format!("{:.2} MB", size_mb), duration))
        })
        .collect();
    vec
}


pub fn total_duration(path: &Path) -> String {
    let video_files = get_video_files(Path::new(path));
    let total_sec: f64 = video_files.iter()
        .map(|(_, _, duration)| {
            let time_parts: Vec<&str> = duration.split(":").collect();
            let hrs: f64 = time_parts[0].parse().unwrap();
            let min: f64 = time_parts[1].parse().unwrap();
            let sec: f64 = time_parts[2].parse().unwrap();
            hrs * 3600.0 + min * 60.0 + sec
        })
        .sum();

    let hrs = (total_sec / 3600.0).floor();
    let min = (total_sec % 3600.0 / 60.0).floor();
    let sec = total_sec % 60.0;

    format!("{:02}:{:02}:{:.2}", hrs as u64, min as u64, sec)
}