use clap::Parser;
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(clap::ValueEnum, Debug, Clone)]
enum FileType {
    F, // Files
    D, // Directory
    L, // Symlink
}

#[derive(Parser, Debug)]
#[command(author="Shabari K S", version, about= "Fast file finder build with Rust")]
struct Args {
    /// Directory to search in
    #[arg(short, long, default_value = ".", help="Directory path")]
    path: PathBuf,
    
    /// Name pattern to search for (e.g., "main.rs)
    #[arg(short, long, help="Name or patterto search for")]
    name: Option<String>,
    
    /// Filter by Extension (e.g., "rs", "txt")
    #[arg(short, long)]
    extension: Option<String>,

    /// Filter by type: f (file), d (directory), l(symlink)
    #[arg(short='t', long="type")]
    file_type: Option<FileType>,

    /// Filter by minimum size (bytes), e.g. 1048576 for 1 MB
    #[arg(short = 's', long)]
    min_size: Option<u64>,

    /// Filter by maximum size (bytes)
    #[arg(long)]
    max_size: Option<u64>
}

fn main() {
    let args = Args::parse();

    println!("Searching in: {:?}", args.path);

    // 1. Collect all files (WalkDir is serial, but fast)
    let entries: Vec<_> = WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())  // Ignore permission errors
        .collect();

    // 2. Process/Search in Paralll using Rayon
    //  par_iter() auomatical splits work across all CP cores
    let matches: Vec<_> = entries.par_iter()
        .filter(|entry| {
            let path = entry.path();

            // Check Name Match
            if let Some(target_name) = &args.name {
                if let Some(file_name) = path.file_name() {
                    if !file_name.to_string_lossy().contains(target_name) {
                        return false;
                    }
                }
            }

            // Check Extension Match
            if let Some(target_ext) = &args.extension {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy() != *target_ext {
                        return false;
                    } 
                }else {
                        return false; // No extension
                }
            }

            if let Some(ft) = &args.file_type {
                let is_match = match ft {
                    FileType::F => entry.file_type().is_file(),
                    FileType::D => entry.file_type().is_dir(),
                    FileType::L => entry.file_type().is_symlink(),
                };
                if !is_match { return false; }
            }

            // Check Size (Only for files)
            if args.min_size.is_some() || args.max_size.is_some() {
                let metadata = entry.metadata().ok(); // Metadata can fail
                if let Some(m) = metadata {
                    let size = m.len();
                    if let Some(min) = args.min_size {
                        if size < min { return false; }
                    }
                    if let Some(max) = args.max_size {
                        if size > max { return false; }
                    }
                } else {
                    return false; // Could not read metadata
                }
            }

            true // Passed all checks
        })
        .collect();

    // 3. Print Results {
    for m in matches {
        println!("{}", m.path().display());
    }
}
