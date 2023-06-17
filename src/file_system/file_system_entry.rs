use std::fs::Metadata;

use chrono::{Date, DateTime, Local};

use crate::file_system::file_system_entry_permission::FileSystemEntryPermission;
use crate::file_system::file_system_entry_type::FileSystemEntryType;
use crate::formatter::Format;

pub(crate) struct FileSystemEntry {
    pub path: String,
    pub entry_type: FileSystemEntryType,
    pub permissions: Vec<FileSystemEntryPermission>,
    pub modified_date: DateTime<Local>,
}

impl FileSystemEntry {
    pub fn from_metadata(path: String, metadata: &Metadata) -> FileSystemEntry {
        let mut date: DateTime<Local> = Default::default();

        if let Ok(modified_date) = metadata.modified() {
            date = DateTime::from(modified_date)
        }

        return FileSystemEntry {
            path: path,
            entry_type: FileSystemEntryType::from_metadata(metadata),
            permissions: FileSystemEntryPermission::from_metadata(metadata),
            modified_date: date,
        };
    }
}
