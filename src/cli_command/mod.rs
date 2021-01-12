use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use std::io::Result;

pub struct  CliCommand {
    pub ConstructTransactionCommand: String,
    pub Utils: String,
}

impl CliCommand {
    pub fn choose_command(self) -> Result<()> {
        let commands:Vec<String> = vec![self.ConstructTransactionCommand, self.Utils];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your action")
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr());
        // println!("***** {:?}", commands[selection.unwrap()]);
        // let s = selection.unwrap();
        match selection.unwrap() {
            Some(0) => println!("============== {}", commands[0]),
            Some(1) => println!("++++++++++++++ {}", commands[1]),
            _ => println!("-----------"),
        };
        Ok(())
    }
}
