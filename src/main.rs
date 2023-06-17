use std::fmt::Formatter;

use colored::Colorize;

use crate::{
    args::arg_param::arg_param_vec::ArgParamVec, args::arg_param::ArgParam,
    file_system::file_system_entry_type::FileSystemEntryType, file_system::FileSystem,
};
use crate::formatter::Format;

mod args;
mod file_system;
mod formatter;

fn main() {
    let args = args::Args {};
    let arguments: Vec<ArgParam> = args.get_args();

    for arg in &arguments {
        match arg {
            ArgParam::ShowDirectoriesOnly => println!("ShowDirectoriesOnly: {}", arg),
            ArgParam::ShowFilesOnly => println!("ShowFilesOnly: {}", arg),
            ArgParam::Value(value) => println!("Value: {}", value),
            ArgParam::Executable(value) => println!("Executable: {}", value),
            ArgParam::None => println!("None: {}", arg),
        }
    }

    let paths = FileSystem::get_current_dir_paths();
    let table_header = Format::table_header();
    println!("{}", table_header);
    for path in paths {
        let formatted_entry = Format::file_system_entry(&path, true);
        println!("{formatted_entry}");
    }
}
