use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct DeleteAccount {
    account_id: String,
}
