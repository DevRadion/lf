use chrono::{DateTime, Local, TimeZone};
use colored::{Color, ColoredString, Colorize, Styles};
use colored::Color::TrueColor;
use colored::Style;

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
        let delimiter = "-".repeat(80);
        format!("{delimiter}\n{:20} {:20} {:20} {:20}\n{delimiter}", "Permission", "Modified: Date", "Time", "Path")
    }

    pub fn file_system_entry(entry: &FileSystemEntry, is_colored: bool) -> String {
        let formatted_permissions = Format::file_system_entry_permission(&entry.permissions);
        let time_string = Format::time(&entry.modified_date);
        let date_string = Format::date(&entry.modified_date);
        let mut formatted = format!("{:20} {:20} {:20} {:20}", formatted_permissions, date_string, time_string, entry.path);

        if is_colored {
            let style = Format::get_style_for_entry(&entry.entry_type);
            formatted = Format::apply_style(&formatted, &style).to_string();
        }

        return formatted;
    }

    /* Private */
    fn get_style_for_entry(entry_type: &FileSystemEntryType) -> FormatTextStyle {
        return match entry_type {
            FileSystemEntryType::File => {
                FormatTextStyle {
                    foreground_color: Color::TrueColor { r: 0, g: 255, b: 136 },
                    background_color: Option::None,
                    style: Styles::Bold,
                }
            }
            FileSystemEntryType::Directory => {
                FormatTextStyle {
                    foreground_color: Color::Green,
                    background_color: Option::None,
                    style: Styles::Bold,
                }
            }
            FileSystemEntryType::Unknown => {
                FormatTextStyle {
                    foreground_color: Color::Red,
                    background_color: Option::None,
                    style: Styles::Bold,
                }
            }
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
