use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// ディレクトリーを結合する関数
fn merge_dir(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            merge_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
            println!("Merged {} to {}", src_path.display(), dst_path.display());
        }
    }
    Ok(())
}

/// ディレクトリーを分割する関数
fn unmerge_dir(merged_folder: &Path, re_chunk_list: &str, sub_chunk_list: &str) -> io::Result<()> {
    let re_chunk_file = File::open(re_chunk_list)?;
    let re_chunk_reader = BufReader::new(re_chunk_file);

    let sub_chunk_file = File::open(sub_chunk_list)?;
    let sub_chunk_reader = BufReader::new(sub_chunk_file);

    for line in re_chunk_reader.lines() {
        let line = line?;
        let src_path = merged_folder.join(&line);
        let dst_path = Path::new("re_chunk_000").join(&line);
        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&src_path, &dst_path)?;
        println!("Unmerged {} to {}", src_path.display(), dst_path.display());
    }

    for line in sub_chunk_reader.lines() {
        let line = line?;
        let src_path = merged_folder.join(&line);
        let dst_path = Path::new("re_chunk_000.pak.sub_000").join(&line);
        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&src_path, &dst_path)?;
        println!("Unmerged {} to {}", src_path.display(), dst_path.display());
    }

    Ok(())
}

/// リストファイルを分割する関数
fn split_list_file(
    input_file: &str,
    re_chunk_dir: &Path,
    sub_chunk_dir: &Path,
    re_chunk_file: &str,
    sub_chunk_file: &str,
) -> io::Result<()> {
    let input = File::open(input_file)?;
    let reader = BufReader::new(input);

    let mut re_chunk_output = File::create(re_chunk_file)?;
    let mut sub_chunk_output = File::create(sub_chunk_file)?;

    for line in reader.lines() {
        let line = line?;
        let relative_path = Path::new(&line); // リストファイル内の相対パス

        // `re_chunk_000` で照合
        let re_chunk_path = re_chunk_dir.join(relative_path);
        if re_chunk_path.exists() {
            writeln!(re_chunk_output, "{}", line)?;
            continue;
        }

        // `re_chunk_000.pak.sub_000` で照合
        let sub_chunk_path = sub_chunk_dir.join(relative_path);
        if sub_chunk_path.exists() {
            writeln!(sub_chunk_output, "{}", line)?;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: MHWs-Ext-Manager.exe <merge|unmerge|split> <args>");
        std::process::exit(1);
    }

    let command = &args[1];
    match command.as_str() {
        "merge" => {
            if args.len() != 4 {
                eprintln!("Usage: MHWs-Ext-Manager.exe merge <source_folder> <destination_folder>");
                std::process::exit(1);
            }
            let src = Path::new(&args[2]);
            let dst = Path::new(&args[3]);
            if let Err(e) = merge_dir(src, dst) {
                eprintln!("Error during merging: {}", e);
            }
        }
        "unmerge" => {
            if args.len() != 5 {
                eprintln!("Usage: MHWs-Ext-Manager.exe unmerge <merged_folder> <list_file_for_re_chunk> <list_file_for_sub_chunk>");
                std::process::exit(1);
            }
            let merged_folder = Path::new(&args[2]);
            let re_chunk_list = &args[3];
            let sub_chunk_list = &args[4];
            if let Err(e) = unmerge_dir(merged_folder, re_chunk_list, sub_chunk_list) {
                eprintln!("Error during unmerging: {}", e);
            }
        }
        "split" => {
            if args.len() != 3 {
                eprintln!("Usage: MHWs-Ext-Manager.exe split <input_list_file>");
                std::process::exit(1);
            }
            // 入力リストファイル
            let input_file = "MHWs_STM_Beta.list";
            // フォルダパス
            let re_chunk_dir = Path::new("re_chunk_000");
            let sub_chunk_dir = Path::new("re_chunk_000.pak.sub_000");
            // 出力リストファイル
            let re_chunk_file = "re_chunk_000.list";
            let sub_chunk_file = "re_chunk_000.pak.sub_000.list";

            match split_list_file(
                input_file,
                re_chunk_dir,
                sub_chunk_dir,
                re_chunk_file,
                sub_chunk_file,
            ) {
                Ok(_) => println!("List files successfully split into '{}' and '{}'.", re_chunk_file, sub_chunk_file),
                Err(e) => eprintln!("Error splitting list file: {}", e),
            }
        }
        _ => eprintln!("Invalid command: {}", command),
    }
}
