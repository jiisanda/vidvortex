# vidvortex

A command line utility for retrieving total video length in directory and its subdirectories and do more iguess🤔.

## Features
- [x] List video files in the current directory and subdirectories
- [x] Set the path to search for video files
- [x] Show total duration of all video files in a directory and its subdirectories
- [ ] Show total duration of all video files in a directory - restricted
- [ ] Convert video files to different format
- [ ] Merge video files

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/learn/get-started)
- [FFmpeg](https://ffmpeg.org/download.html)
- It's suggest to use git bash for running the commands.

FFmpeg is required for the program to work. You can install it by following the instructions on the [official website](https://ffmpeg.org/download.html).
or can run 
```bash
choco install ffmpeg
```

### Clone the repository
```bash
git clone https://github.com/jiisanda/vidvortex.git
```
```bash
cd vidvortex
```

### Build the project

Set source for vidvortex
```bash
source .bashrc
```

Build the project
```bash
vidvortex
```

If not using GitBash, you can run the following command to build the project and add the binary to your PATH.
```bash
cargo build --release
```
```bash
export PATH=$PATH:/path/to/vidvortex/target/release
```

## Usage

```bash
vidvortex [FLAGS] [OPTIONS]
```

Get list of all available arguments, flags and subcommands.
```bash
vidvortex --help
```

Output will be something like this:
```bash
Options:
    -h, --help          list of all available arguments
    -v, --version       check version of vidvortex
    -l, --list          list video files in the current directory and
                        subdirectories
    -p, --path PATH     set the path to search for video files, default
                        Current directory
    -t, --total         show total duration of all video files
```

Try it out yourself! to view how it works.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
