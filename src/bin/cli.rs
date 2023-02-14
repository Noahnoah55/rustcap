use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short='o')]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let maybepic = rustcap::screenshot();
    let pic = match maybepic {
        Ok(p) => p,
        Err(e) => panic!("Failed to take screenshot: {}", e)
    };
    let path: PathBuf; 
    if let Some(p) = args.path {
        path = p;
    }
    else {
        path = default_path();
    }
    if let Err(e) = pic.save(&path) {
        panic!("Failed to save file to path \"{:0}\": {:1}", path.to_string_lossy(), e.to_string())
    }
    else {
        println!("Successfully saved screenshot to file://{}", path.to_string_lossy());
    }
}

fn default_path() -> PathBuf {
    let now = chrono::Local::now();
    let now = now.to_rfc3339();
    // TODO: Get rid of this unwrap, probably read default directory from a file
    let mut path: PathBuf = dirs::home_dir().unwrap();
    path.push("Pictures");
    path.push("Rustcap");
    std::fs::create_dir_all(&path).expect("Failed to save screencap");
    path.push(now.to_string());
    path.set_extension("png");
    return path;
}