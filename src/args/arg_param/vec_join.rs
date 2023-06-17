use crate::args::arg_param::ArgParam;

pub(crate) trait Join {
    fn join(&self, separator: &str) -> String;
}

impl Join for &Vec<ArgParam> {
    fn join(&self, separator: &str) -> String {
        if self.iter().count() == 1 {
            return self.first().unwrap_or(&ArgParam::None).raw_value();
        }

        let mut result = String::new();
        for (index, item) in self.iter().enumerate() {
            println!("{index}");
            if index > 0 {
                result.push_str(separator);
            }
            result.push_str(&item.to_string());
        }
        result
    }
}
