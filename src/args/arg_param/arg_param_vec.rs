use crate::args::arg_param::ArgParam;
use crate::args::arg_param::vec_join::Join;

pub(crate) trait ArgParamVec {
    fn get_value_for<'a>(
        &'a self,
        arg: &ArgParam,
        args: &'a Vec<ArgParam>,
    ) -> Result<&ArgParam, String>;
    fn contains_arg(&self, arg: ArgParam) -> bool;
}

impl ArgParamVec for Vec<ArgParam> {
    fn get_value_for<'a>(
        &'a self,
        arg: &ArgParam,
        args: &'a Vec<ArgParam>,
    ) -> Result<&ArgParam, String> {
        let arg_idx = args
            .iter()
            .position(|elem| elem.raw_value() == arg.raw_value());
        if let Some(idx) = arg_idx {
            if let Some(value) = args.get(idx + 1) {
                return Ok(value);
            }
        }

        return Err(format!(
            "Args: [{}] does not contain arg: {}",
            args.join(", "),
            arg.raw_value()
        ));
    }

    fn contains_arg(&self, arg: ArgParam) -> bool {
        for _arg in self {
            if _arg.raw_value() == arg.raw_value() {
                return true;
            }
        }
        return false;
    }
}
