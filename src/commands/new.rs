
pub fn exec(args: &Vec<String>) -> Result<String, String> {
    for a in args.iter() {
        println!("{}", a);
    }
    Ok("Hello from new".to_string())
}

