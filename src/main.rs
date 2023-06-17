use std::fmt::Formatter;
use std::fs::FileType;

use colored::Colorize;

use crate::formatter::Format;
use crate::{
    args::arg_param::arg_param_vec::ArgParamVec, args::arg_param::ArgParam,
    file_system::file_system_entry_type::FileSystemEntryType, file_system::FileSystem,
};

mod args;
mod file_system;
mod formatter;

fn main() {
    let args = args::Args {};
    let arguments: Vec<ArgParam> = args.get_args();
    //
    // for arg in &arguments {
    //     match arg {
    //         ArgParam::ShowDirectoriesOnly => println!("ShowDirectoriesOnly: {}", arg),
    //         ArgParam::ShowFilesOnly => println!("ShowFilesOnly: {}", arg),
    //         ArgParam::Value(value) => println!("Value: {}", value),
    //         ArgParam::Executable(value) => println!("Executable: {}", value),
    //         ArgParam::None => println!("None: {}", arg),
    //     }
    // }

    let mut paths = FileSystem::get_current_dir_paths();
    let table_header = Format::table_header();
    println!("{}", table_header);

    let is_show_dir_only = arguments.contains_arg(ArgParam::ShowDirectoriesOnly);
    let is_show_files_only = arguments.contains_arg(ArgParam::ShowFilesOnly);
    paths.sort();
    for path in paths {
        if is_show_dir_only && path.entry_type == FileSystemEntryType::Directory
            || is_show_files_only && path.entry_type == FileSystemEntryType::File
            || !is_show_dir_only && !is_show_files_only
        {
            let formatted_entry = Format::file_system_entry(&path, true);
            println!("{formatted_entry}");
        }
    }
}
