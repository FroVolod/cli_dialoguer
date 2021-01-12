use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};

use std::io::Result;

use create_account_command;
use delete_account_command;
use send_command;
use stake_command;

pub struct ConstructTransactionCommand {
    TransferNEARTokens: send_command::Send,
    CallFunction: String,
    StakeNEARTokens: stake_command::Stake,
    CreateAccount: create_account_command::CreateAccount,
    DeleteAccount: delete_account_command::DeleteAccount,
    // Add an Access Key
    // Detete an Access Key
}
