use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use std::io::Result;

mod select_server;

pub struct OnOffLine {
    pub Yes: String,
    pub No: String
}

impl OnOffLine {
    pub fn choose_action(self) -> Result<()> {
        let commands:Vec<String> = vec![self.Yes, self.No];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to derive some information required for transaction construction automatically querying it online?")
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        let select_server = select_server::SelectServer {
            Testnet: String::from("testnet (https://rpc.testnet.near.org)"),
            Mainnet: String::from ("mainnet (https://rpc.mainnet.near.org)"),
            Betanet: String::from ("betanet (https://rpc.betanet.near.org)"),
            Custom: String::from ("Custom (will prompt you to provide RPC URL)"),
        };
        match selection {
            Some(0) => {
                println!("============== {}", commands[0]);
                select_server.select_server();
            },
            Some(1) => println!("++++++++++++++ {}", commands[1]),
            _ => println!("-----------"),
        };
        Ok(())
    }
}
