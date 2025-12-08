extern crate fs_extra;
use fs_extra::dir::get_size;
use std::{
    collections::BTreeSet,
    fs::{self, DirEntry, ReadDir},
    path::PathBuf,
};

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

#[derive(Eq)]
struct ProgramFolder {
    bytes: u64,
    path: String,
}

impl Ord for ProgramFolder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.bytes > other.bytes {
            return std::cmp::Ordering::Greater;
        }
        if self.bytes < other.bytes {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for ProgramFolder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.bytes.partial_cmp(&other.bytes) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.path.partial_cmp(&other.path)
    }
}

impl PartialEq for ProgramFolder {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes && self.path == other.path
    }
}

struct Unit<'a> {
    size: u64,
    name: &'a str,
}

const GB: Unit = Unit {
    size: 1 << 30,
    name: "GB",
};

const MB: Unit = Unit {
    size: 1 << 20,
    name: "MB",
};

const KB: Unit = Unit {
    size: 1 << 10,
    name: "KB",
};

fn format_size(bytes: u64) -> String {
    match [GB, MB, KB].iter().find(|unit| bytes > unit.size) {
        Some(unit) => {
            let num_units = bytes / unit.size;
            return num_units.to_string()
                + " "
                + &unit.name
                + " "
                + &format_size(bytes - num_units * unit.size);
        }
        None => return bytes.to_string() + " bytes",
    };
}

fn main() {
    let limit = 20;
    let mut found = BTreeSet::<ProgramFolder>::new();
    let path = PathBuf::from("./")
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    find_biggest_programs(&path, &mut found, limit);
    for folder in found {
        println!("{} with {}", folder.path, format_size(folder.bytes));
    }
}
