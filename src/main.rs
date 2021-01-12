use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use std::io::Result;

struct CliArgs {
    subcommand: CliCommand,
}

struct  CliCommand {
    ConstructTransactionCommand: String,
    Utils: String,
}

impl CliCommand {
    fn choose_command(self) -> Result<()> {
        let commands:Vec<String> = vec![self.ConstructTransactionCommand, self.Utils];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr());
        println!("***** {:?}", selection.unwrap());
        // let s = selection.unwrap();
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli_command = CliCommand {
        ConstructTransactionCommand: "construct-transaction-command".to_string(),
        Utils: "utils".to_string(),
    };
    cli_command.choose_command();

    Ok(())
}
