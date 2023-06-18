use std::fs::Metadata;

use crate::file_system::file_system_entry_type::FileSystemEntryType::File;

pub(crate) enum FileSystemEntryType {
    File,
    Directory,
    Unknown,
}

impl PartialEq for FileSystemEntryType {
    fn eq(&self, other: &Self) -> bool {
        self.raw_value() == other.raw_value()
    }

    fn ne(&self, other: &Self) -> bool {
        self.raw_value() != other.raw_value()
    }
}

impl FileSystemEntryType {
    pub fn from_metadata(metadata: &Metadata) -> FileSystemEntryType {
        return if metadata.is_dir() {
            FileSystemEntryType::Directory
        } else if metadata.is_file() {
            FileSystemEntryType::File
        } else {
            FileSystemEntryType::Unknown
        };
    }

    pub fn raw_value(&self) -> String {
        match self {
            FileSystemEntryType::File => "File".to_string(),
            FileSystemEntryType::Directory => "Directory".to_string(),
            FileSystemEntryType::Unknown => "Unknown".to_string(),
        }
    }

    pub fn get_ordering_id(&self) -> i8 {
        if self.raw_value() == File.raw_value() {
            return 1;
        } else {
            return 0;
        }
    }
}
