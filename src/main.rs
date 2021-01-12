use std::io::Result;

mod cli_command;


fn main() -> Result<()> {
    println!();
    let cli_command = cli_command::CliCommand {
        ConstructTransactionCommand: "construct-transaction-command".to_string(),
        Utils: "utils".to_string(),
    };
    cli_command.choose_command();

    Ok(())
}
