pub fn validate(val : &str, no_validation: bool) -> (String, bool) {
    if val.is_empty() || no_validation {
        //all ok
        return ("".to_string(), true)
    }
    if let Err(err) = val.parse::<f64>() {
        (format!("{}", err), false)
    } else {
        ("".to_string(), true)
    }
}