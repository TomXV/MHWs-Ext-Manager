use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn merge_directories(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            merge_directories(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
            println!("Merged {} to {}", src_path.display(), dst_path.display());
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: MergePak.exe <source_folder> <destination_folder>");
        std::process::exit(1);
    }

    let src_folder = Path::new(&args[1]);
    let dst_folder = Path::new(&args[2]);

    if !src_folder.exists() || !src_folder.is_dir() {
        eprintln!("Source folder does not exist or is not a directory: {}", src_folder.display());
        std::process::exit(1);
    }

    if !dst_folder.exists() || !dst_folder.is_dir() {
        eprintln!("Destination folder does not exist or is not a directory: {}", dst_folder.display());
        std::process::exit(1);
    }

    match merge_directories(src_folder, dst_folder) {
        Ok(_) => println!("All files merged successfully!"),
        Err(e) => eprintln!("Error during merging: {}", e),
    }
}
