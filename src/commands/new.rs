use std::env::home_dir;

pub fn exec(args: &Vec<String>) -> Result<String, String> {
    if args.len() != 2 {
        return Err("Usage: snpts new <name> [path]".to_string());
    }

    // let home = match home_dir() {
    //     Some (mut p) => { p.push(".snpts.db") },
    //     None => { panic!("Can't open db file!") }
    // };


    // let name = args[0].clone();
    // let path = args[1].clone();

    Ok("Hello from new".to_string())
}

