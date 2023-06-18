use std::cmp::Ordering;
use std::fs::Metadata;
#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

use chrono::{DateTime, Local};

use crate::file_system::file_system_entry_permission::FileSystemEntryPermission;
use crate::file_system::file_system_entry_type::FileSystemEntryType;

pub(crate) struct FileSystemEntry {
    pub path: String,
    pub entry_type: FileSystemEntryType,
    pub permissions: Vec<FileSystemEntryPermission>,
    pub modified_date: DateTime<Local>,
    pub size: u64,
}

impl FileSystemEntry {
    pub fn from_metadata(path: String, metadata: &Metadata) -> FileSystemEntry {
        let mut date: DateTime<Local> = Default::default();

        if let Ok(modified_date) = metadata.modified() {
            date = DateTime::from(modified_date)
        }

        let file_size: u64;
        #[cfg(target_os = "macos")] { file_size = metadata.len() }
        #[cfg(target_os = "windows")] { file_size = metadata.file_size() }

        return FileSystemEntry {
            path: path,
            entry_type: FileSystemEntryType::from_metadata(metadata),
            permissions: FileSystemEntryPermission::from_metadata(metadata),
            modified_date: date,
            size: file_size,
        };
    }
}

impl Eq for FileSystemEntry {}

impl PartialEq<Self> for FileSystemEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PartialOrd<Self> for FileSystemEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileSystemEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entry_type.get_ordering_id().cmp(&other.entry_type.get_ordering_id())
    }
}
