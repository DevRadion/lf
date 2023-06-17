use std::fmt::{Display, Formatter};

pub(crate) mod arg_param_vec;
pub(crate) mod vec_join;

pub(crate) enum ArgParam {
    ShowDirectoriesOnly,
    ShowFilesOnly,
    Value(String),
    Executable(String),
    None,
}

impl ArgParam {
    pub fn raw_value(&self) -> String {
        String::from(match self {
            ArgParam::ShowDirectoriesOnly => "-d",
            ArgParam::ShowFilesOnly => "-f",
            ArgParam::Value(value) => value,
            ArgParam::Executable(value) => value,
            ArgParam::None => "",
        })
    }

    pub fn from_string(raw_value: &String) -> ArgParam {
        match raw_value.as_str() {
            "-d" => ArgParam::ShowDirectoriesOnly,
            "-f" => ArgParam::ShowFilesOnly,
            "" => ArgParam::None,
            _ => {
                if raw_value.contains("lf") {
                    ArgParam::Executable(raw_value.to_string())
                } else {
                    ArgParam::Value(raw_value.to_string())
                }
            }
        }
    }
}

impl Display for ArgParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw_value())
    }
}
