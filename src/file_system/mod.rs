use std::fs;

use crate::file_system::file_system_entry::FileSystemEntry;

pub(crate) mod file_system_entry;
pub(crate) mod file_system_entry_permission;
pub(crate) mod file_system_entry_type;

pub(crate) struct FileSystem;

impl FileSystem {
    pub(crate) fn get_current_dir_paths() -> Vec<FileSystemEntry> {
        let paths_result = fs::read_dir("./");
        let mut entries: Vec<FileSystemEntry> = vec![];

        if let Ok(paths) = paths_result {
            for path_entry in paths {
                if let Ok(path_entry) = path_entry {
                    let path_entry = path_entry.path();
                    let path = path_entry.to_str().unwrap_or("");
                    if path.is_empty() {
                        continue;
                    }

                    if let Ok(metadata) = path_entry.metadata() {
                        entries.push(FileSystemEntry::from_metadata(path.to_string(), &metadata));
                    }
                }
            }
        }

        return entries;
    }
}
