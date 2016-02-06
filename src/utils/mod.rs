use ::regex::Regex;

pub mod db;

pub fn insert_vars(snippet: &str, vars: &Vec<&str>) -> String {
    let re = Regex::new(r"(\$\d)").unwrap();
    let mut res = snippet.to_string();
    for i in 0..(re.captures_len() + 1) {
        res = re.replace(&res, vars[i]);
    }
    return res;
}


#[test]
fn instert_vars_basic_test() {
    assert_eq!(insert_vars("Hello, $1! Is there $2 out $3?", &vec!["world", "anybody", "there"]), "Hello, world! Is there anybody out there?");
}
