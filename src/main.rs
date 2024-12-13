use structopt::StructOpt;
mod commands;

#[derive(Debug, StructOpt)]
enum Opt {
    BurnAddress,
    Burn,
    Vote,
    Verify
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::BurnAddress => {
            commands::burn_address::burn_address().await
        }
        Opt::Burn => {
            commands::burn::burn().await
        }
        Opt::Vote => {
            commands::vote::vote().await
        }
        Opt::Verify => {
            commands::verify::verify().await
        }
    }
}
