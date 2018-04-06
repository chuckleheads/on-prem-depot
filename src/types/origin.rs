use rocket::request::FromParam;
use std::borrow::Cow;
use rocket::http::RawStr;

pub struct OriginName<'a>(Cow<'a, str>);

fn valid_origin(origin: &str) -> bool {
    origin.chars().all(|c| c.is_alphanumeric())
}

impl<'a> FromParam<'a> for OriginName<'a> {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<OriginName<'a>, &'a RawStr> {
        match valid_origin(param) {
            true => Ok(OriginName(param.percent_decode_lossy())),
            false => Err(param),
        }
    }
}

impl<'a> ToString for OriginName<'a> {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}
