use ::commands::{Named, Executable};

pub struct NewCommand {
    name: String
}

impl NewCommand {
    pub fn new() -> NewCommand {
        NewCommand { name: "new".to_string() }
    }
}


impl Named for NewCommand {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Executable for NewCommand {
    fn exec(&self, args: &Vec<String>) -> Result<String, String> {
        Ok("Hello from new".to_string())
    }
}
