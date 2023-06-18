use std::collections::HashMap;
use crate::formatter::Format;
use crate::{
    args::arg_param::arg_param_vec::ArgParamVec, args::arg_param::ArgParam,
    file_system::file_system_entry_type::FileSystemEntryType, file_system::FileSystem,
};

mod args;
mod file_system;
mod formatter;

fn print_file_list(arguments: &Vec<ArgParam>) {
    let mut paths = FileSystem::get_current_dir_paths();
    let table_header = Format::table_header();
    println!("{}", table_header);

    let is_show_dir_only = arguments.contains_arg(ArgParam::ShowDirectoriesOnly);
    let is_show_files_only = arguments.contains_arg(ArgParam::ShowFilesOnly);
    paths.sort();
    for path in &paths {
        if is_show_dir_only && path.entry_type == FileSystemEntryType::Directory
            || is_show_files_only && path.entry_type == FileSystemEntryType::File
            || !is_show_dir_only && !is_show_files_only
        {
            let formatted_entry = Format::file_system_entry(&path, true);
            println!("{formatted_entry}");
        }
    }

    if paths.iter().count() > 20 {
        println!("{}", table_header);
    }
}

fn print_help_info() {
    let info: HashMap<ArgParam, String> = HashMap::from([
        (ArgParam::Help, "Prints usage information".to_string()),
        (ArgParam::ShowDirectoriesOnly, "Shows only directories".to_string()),
        (ArgParam::ShowFilesOnly, "Shows only files".to_string()),
        (ArgParam::Verbose, "Prints verbose info (now, it's arguments that used)".to_string()),
    ]);

    let formatted_info = Format::help_info(info);
    println!("{formatted_info}");
}

fn print_verbose_info(arguments: &Vec<ArgParam>) {
    println!("Runned in Verbose mode -");
    for arg in arguments {
        match arg {
            ArgParam::ShowDirectoriesOnly => println!("ShowDirectoriesOnly: {}", arg),
            ArgParam::ShowFilesOnly => println!("ShowFilesOnly: {}", arg),
            ArgParam::Value(value) => println!("Value: {}", value),
            ArgParam::Executable(value) => println!("Executable: {}", value),
            ArgParam::Verbose => println!("Verbose: {}", arg),
            ArgParam::Help => println!("Help: {}", arg),
            ArgParam::None => println!("None: {}", arg),
        }
    }
}

fn main() {
    let args = args::Args {};
    let arguments: Vec<ArgParam> = args.get_args();

    if arguments.contains_arg(ArgParam::Verbose) {
        print_verbose_info(&arguments);
    }

    if !arguments.contains_arg(ArgParam::Help) {
        print_file_list(&arguments);
    } else {
        print_help_info();
    }
}
