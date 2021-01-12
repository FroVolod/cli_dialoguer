use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use std::io::Result;



pub struct SelectServer {
    pub Testnet: String,
    pub Mainnet: String,
    pub Betanet: String,
    pub Custom: String,
}

impl SelectServer {
    pub fn select_server(self) -> Result<()> {
        let commands:Vec<String> = vec![self.Testnet, self.Mainnet, self.Betanet, self.Custom];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select NEAR protocol RPC server:")
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(0) => {
                println!("============== {}", commands[0]);
                
            },
            Some(1) => println!("++++++++++++++ {}", commands[1]),
            Some(2) => println!("++++++++++++++ {}", commands[2]),
            Some(3) => println!("++++++++++++++ {}", commands[3]),
            _ => println!("-----------"),
        };
        Ok(())
    }
}
