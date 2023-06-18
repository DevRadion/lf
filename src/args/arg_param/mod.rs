use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

pub(crate) mod arg_param_vec;
pub(crate) mod vec_join;

pub(crate) enum ArgParam {
    ShowDirectoriesOnly,
    ShowFilesOnly,
    Value(String),
    Executable(String),
    Verbose,
    Help,
    None,
}

impl ArgParam {
    pub fn raw_value(&self) -> String {
        String::from(match self {
            ArgParam::ShowDirectoriesOnly => "-d",
            ArgParam::ShowFilesOnly => "-f",
            ArgParam::Value(value) => value,
            ArgParam::Executable(value) => value,
            ArgParam::Verbose => "-v",
            ArgParam::Help => "--help",
            ArgParam::None => "",
        })
    }

    pub fn from_string(raw_value: &String) -> ArgParam {
        match raw_value.as_str() {
            "-d" => ArgParam::ShowDirectoriesOnly,
            "-f" => ArgParam::ShowFilesOnly,
            "-v" => ArgParam::Verbose,
            "--help" => ArgParam::Help,
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

impl Hash for ArgParam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ArgParam::ShowDirectoriesOnly => {
                state.write_u8(0);
            }
            ArgParam::ShowFilesOnly => {
                state.write_u8(1);
            }
            ArgParam::Value(value) => {
                state.write_u8(2);
                value.hash(state);
            }
            ArgParam::Executable(value) => {
                state.write_u8(3);
                value.hash(state);
            }
            ArgParam::Verbose => {
                state.write_u8(4);
            }
            ArgParam::Help => {
                state.write_u8(5);
            }
            ArgParam::None => {
                state.write_u8(6);
            }
        }
    }
}

impl PartialEq<Self> for ArgParam {
    fn eq(&self, other: &Self) -> bool {
        self.raw_value() == other.raw_value()
    }
}

impl Eq for ArgParam {

}

impl Display for ArgParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw_value())
    }
}
