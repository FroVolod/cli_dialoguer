use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CreateAccount {
    account_id: String,
    }
