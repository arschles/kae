pub type Res = std::result::Result<(), String>;

pub fn err_res(s: &str) -> Res {
    Err(String::from(s))
}
