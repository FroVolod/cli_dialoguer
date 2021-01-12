use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Send {
    #[structopt(long)]  // Можно удалить строку. Тогда субкоманда будет работать без "флага".
    account_id: String,
    #[structopt(long)]
    receiver_id: String,
    #[structopt(long)]
    amount: u128,
}
