use std::cmp::Ordering;
use std::fs::Metadata;

#[derive(Clone)]
pub(crate) enum FileSystemEntryPermission {
    Read,
    Write,
    ReadOnly,
}

impl Eq for FileSystemEntryPermission {}

impl PartialEq<Self> for FileSystemEntryPermission {
    fn eq(&self, other: &Self) -> bool {
        self.get_ordering_id() == other.get_ordering_id()
    }
}

impl PartialOrd<Self> for FileSystemEntryPermission {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileSystemEntryPermission {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_ordering_id().cmp(&other.get_ordering_id())
    }
}

impl FileSystemEntryPermission {
    pub fn raw_value(&self) -> String {
        String::from(match self {
            FileSystemEntryPermission::Read => "Read",
            FileSystemEntryPermission::Write => "Write",
            FileSystemEntryPermission::ReadOnly => "ReadOnly",
        })
    }

    pub fn from_metadata(metadata: &Metadata) -> Vec<FileSystemEntryPermission> {
        let permissions = metadata.permissions();
        let readonly = permissions.readonly();
        let writable = !readonly;

        let mut entry_permissions = Vec::new();
        if readonly {
            entry_permissions.push(FileSystemEntryPermission::ReadOnly);
        } else {
            if writable {
                entry_permissions.push(FileSystemEntryPermission::Read);
                entry_permissions.push(FileSystemEntryPermission::Write);
            } else {
                entry_permissions.push(FileSystemEntryPermission::Read);
            }
        }

        entry_permissions
    }

    fn get_ordering_id(&self) -> i8 {
        match self {
            FileSystemEntryPermission::Read => 0,
            FileSystemEntryPermission::Write => 1,
            FileSystemEntryPermission::ReadOnly => 2,
        }
    }
}
