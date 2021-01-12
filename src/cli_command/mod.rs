use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use on_off_line::OnOffLine;
use std::io::Result;

mod on_off_line;

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
            .interact_on_opt(&Term::stderr())
            .unwrap();

        let on_off_line = on_off_line::OnOffLine {
            Yes: String::from("Yes, I keep it simple"),
            No: String::from("No, I want to work in no-network (air-gapped) environment")
        };

        match selection {
            Some(0) => {
                println!("============== {}", commands[0]);
                on_off_line.choose_action();
            },
            Some(1) => println!("++++++++++++++ {}", commands[1]),
            _ => println!("-----------"),
        };
        Ok(())
    }
}
