use regex::Regex;

pub fn valid_email(email: &str) -> bool {
    let re = Regex::new(r"(\w+)?\.?(\w+)@\w+\.\w+").expect("regex failed to compile");
    re.is_match(email)
}
