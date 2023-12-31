use std::collections::HashMap;
use std::ops::Deref;
use chrono::{DateTime, Local};
use colored::{Color, ColoredString, Colorize, Styles};
use crate::args::arg_param::ArgParam;

use crate::file_system::file_system_entry::FileSystemEntry;
use crate::file_system::file_system_entry_permission::FileSystemEntryPermission;
use crate::file_system::file_system_entry_type::FileSystemEntryType;

pub(crate) struct Format;

struct FormatTextStyle {
    foreground_color: Color,
    background_color: Option<Color>,
    style: Styles,
}

impl Format {
    pub fn date(date_time: &DateTime<Local>) -> String {
        date_time.format("%Y-%m-%d").to_string()
    }

    pub fn time(date_time: &DateTime<Local>) -> String {
        date_time.format("%H:%M:%S").to_string()
    }

    pub fn file_system_entry_permission(permissions: &Vec<FileSystemEntryPermission>) -> String {
        let mut sorted_permissions = permissions.clone();
        sorted_permissions.sort();

        let sorted_str_permissions: Vec<String> = sorted_permissions
            .iter()
            .map(|perm| perm.raw_value())
            .collect();
        let formatted = sorted_str_permissions.join("-");

        return formatted;
    }

    pub fn table_header() -> String {
        let delimiter = "-".repeat(104);
        format!(
            "{delimiter}\n{:^70}\n{delimiter}\n{:20} {:20} {:20} {:20} {:20}\n{delimiter}",
            "Modified", "Permission", "Date", "Time", "Size", "Path"
        )
    }

    pub fn file_system_entry(entry: &FileSystemEntry, is_colored: bool) -> String {
        let formatted_permissions = Format::file_system_entry_permission(&entry.permissions);
        let time_string = Format::time(&entry.modified_date);
        let date_string = Format::date(&entry.modified_date);
        let mut formatted = format!(
            "{:20} {:20} {:20} {:20} {:20}",
            formatted_permissions,
            date_string,
            time_string,
            entry.size.to_string() + " bytes",
            entry.path
        );

        if is_colored {
            let style = Format::get_style_for_entry(&entry.entry_type);
            formatted = Format::apply_style(&formatted, &style).to_string();
        }

        return formatted;
    }

    pub fn help_info(info: HashMap<ArgParam, String>) -> String {
        let delimiter_size = 60;
        let mut formatted = String::new();
        // Add delimiter
        formatted.push_str(("-".repeat(delimiter_size) + "\n").deref());
        // Add usage/example info
        formatted.push_str("Usage: lf [args...]\n");
        formatted.push_str("Example: lf -d -v\n\n");

        // Add args description
        for (arg, description) in info {
            let formatted_arg_info = format!("{arg}: {description}\n");
            formatted.push_str(&formatted_arg_info);
        }

        // Add delimiter
        formatted.push_str(("-".repeat(delimiter_size) + "\n").deref());

        return formatted;
    }

    /* Private */
    fn get_style_for_entry(entry_type: &FileSystemEntryType) -> FormatTextStyle {
        return match entry_type {
            FileSystemEntryType::File => FormatTextStyle {
                foreground_color: Color::TrueColor {
                    r: 0,
                    g: 255,
                    b: 136,
                },
                // None
                background_color: Option::None,
                style: Styles::Bold,
            },
            FileSystemEntryType::Directory => FormatTextStyle {
                foreground_color: Color::Blue,
                background_color: Option::None,
                style: Styles::Bold,
            },
            FileSystemEntryType::Unknown => FormatTextStyle {
                foreground_color: Color::Red,
                background_color: Option::None,
                style: Styles::Bold,
            },
        };
    }

    fn apply_style(text: &String, style: &FormatTextStyle) -> ColoredString {
        let mut styled_text = text.color(style.foreground_color);

        if let Some(background_color) = style.background_color {
            styled_text = styled_text.on_color(background_color);
        }

        return match style.style {
            Styles::Clear => styled_text.clear(),
            Styles::Bold => styled_text.bold(),
            Styles::Dimmed => styled_text.dimmed(),
            Styles::Underline => styled_text.underline(),
            Styles::Reversed => styled_text.reversed(),
            Styles::Italic => styled_text.italic(),
            Styles::Blink => styled_text.blink(),
            Styles::Hidden => styled_text.hidden(),
            Styles::Strikethrough => styled_text.strikethrough(),
        };
    }
}
