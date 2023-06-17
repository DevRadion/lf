use std::fs::Metadata;

pub(crate) enum FileSystemEntryType {
    File,
    Directory,
    Unknown,
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
}
