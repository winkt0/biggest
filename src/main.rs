extern crate fs_extra;
use fs_extra::dir::get_size;
use std::{
    collections::BTreeSet,
    fs::{self, DirEntry, ReadDir},
    path::PathBuf,
};
use clap::{Parser};

mod program_folder;
mod format;
use crate::{program_folder::ProgramFolder, format::format_size};

fn hints_at_program_directory(entry: DirEntry) -> bool {
    let name = entry.file_name().into_string().unwrap().to_uppercase();
    let folders_typically_found_in_program_directories = ["SRC", "BIN", "LOGS", ".GIT"];
    let files_typically_found_in_program_directories = [
        "README", "LICENSE", "LOGS", "VERSION", "LAUNCHER", "SETTINGS",
    ];
    let is_typical_folder_name = folders_typically_found_in_program_directories
        .iter()
        .any(|folder| name.eq_ignore_ascii_case(&folder));
    let is_typical_file_name = files_typically_found_in_program_directories
        .iter()
        .any(|file| name.contains(file) && name.len().abs_diff(file.len()) < 5);
    is_typical_file_name || is_typical_folder_name
}

fn is_program_directory(mut dir: ReadDir) -> bool {
    dir.any(|entry| entry.is_ok_and(hints_at_program_directory))
}

fn find_biggest_programs(current_path: &str, found: &mut BTreeSet<ProgramFolder>, limit: usize) {
    let dir_result = fs::read_dir(current_path);
    if dir_result.is_err() {
        return;
    }

    let current_dir = dir_result.unwrap();
    if is_program_directory(current_dir)
        && let Ok(current_size) = get_size(current_path)
    {
        {
            let folder = ProgramFolder {
                bytes: current_size,
                path: current_path.into(),
            };
            found.insert(folder);
            if found.len() > limit {
                found.pop_first();
            }
        }
        return;
    }

    fs::read_dir(current_path).unwrap().for_each(|entry_res| {
        if let Ok(entry) = entry_res {
            let entry_path =
                current_path.to_owned() + "/" + &entry.file_name().into_string().unwrap();
            find_biggest_programs(&entry_path, found, limit);
        }
    });
}

const DEFAULT_PATH: &str = "./";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Only capture the biggest <LIMIT> folders. 
    #[arg(long, default_value_t=10)]
    limit: usize,
    /// Restrict search to folders within <PATH>
    #[arg(long, default_value_t=DEFAULT_PATH.to_string())]
    path: String
}

fn main() {
    let mut found = BTreeSet::<ProgramFolder>::new();
    let parsed = Args::parse();
    let limit = parsed.limit;
    let path = parsed.path;
    let path = PathBuf::from(path)
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    println!("Looking for the {} biggest program folders within {}...", limit, path);
    find_biggest_programs(&path, &mut found, limit);
    for folder in found.iter().rev() {
        println!("{} with {}", folder.path, format_size(folder.bytes));
    }
}
