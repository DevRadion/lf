use std::env;

use crate::args::arg_param::ArgParam;

pub(crate) mod arg_param;

pub(crate) struct Args;

impl Args {
    pub fn get_args(&self) -> Vec<ArgParam> {
        let args: Vec<String> = env::args().collect();
        return self.vec_string_to_param(args);
    }

    /* MARK: - Private - */

    fn vec_string_to_param(&self, vec: Vec<String>) -> Vec<ArgParam> {
        let arg_param_vec = vec
            .iter()
            .map(|arg_str| ArgParam::from_string(arg_str))
            .collect();

        return arg_param_vec;
    }
}
