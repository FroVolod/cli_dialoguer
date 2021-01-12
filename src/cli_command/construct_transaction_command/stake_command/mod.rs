use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Stake {
    account_id: String,
    amount: u128,
}
