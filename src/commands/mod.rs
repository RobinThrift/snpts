use std::collections::HashMap;

mod new;

pub fn find_and_exec_command(name: &str, args: &Vec<String>) -> Result<String, String> {
    let mut command_map = HashMap::new();
    command_map.insert("new", new::exec);

    match command_map.get(name) {
        Some(exec) => exec(args),
        None => Err("Command not recognised".to_string())
    }
}
