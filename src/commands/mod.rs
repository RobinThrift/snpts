
pub fn exec_command(name: &str) -> Result<&str, &str> {
    if name == "new" {
        return Ok("Hello from new");
    } else {
        return Err("Command not recognised");
    }
}
