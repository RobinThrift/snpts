

pub trait Executable {
    fn exec(&self, args: &Vec<String>) -> Result<String, String>;
}

pub trait Named {
    fn get_name(&self) -> String;
}

mod new;

pub fn find_and_exec_command(name: &str, args: &Vec<String>) -> Result<String, String> {
    match name {
        "new" => new::NewCommand::new().exec(args),
        _ => Err("Command not recognised".to_string())
    }
}
