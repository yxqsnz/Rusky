pub use rusky_core_macros::*;
#[test]
fn basic_command_test() {
    use crate::{commands::*, Result};
    #[command("lol")]
    #[description("?Test")]
    async fn lol_command(_context: &CommandContext) -> Result<()> {
        println!("lol command!");
        Ok(())
    }
    assert_eq!(LOL_COMMAND.data().name, String::from("lol"));
    assert_eq!(LOL_COMMAND.data().description, Some(String::from("?Test")));
}
