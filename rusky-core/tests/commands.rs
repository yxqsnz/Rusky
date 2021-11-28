use rusky_core::{commands::*, macros::command, Result};
#[command("lol")]
#[description("?Test")]
async fn lol_command(_context: CommandContext) -> Result<()> {
    println!("lol command!");
    Ok(())
}

#[test]
pub fn basic_command_name_test() {
    let command_name = LOL_COMMAND.data().name;
    assert_eq!(command_name, String::from("lol"));
}
#[test]
pub fn basic_command_description_test() {
    let command_description = LOL_COMMAND.data().description;
    assert_eq!(command_description, String::from("?Test"));
}
