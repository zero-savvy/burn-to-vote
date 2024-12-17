use structopt::StructOpt;
mod commands;
use commands::burn_address::BurnAddress;

#[derive(Debug, StructOpt)]
enum Opt {
    BurnAddress(BurnAddress),
    Burn,
    Vote,
    Verify,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::BurnAddress(burn_address) => 
        commands::burn_address::burn_address(burn_address ).await,
        Opt::Burn => commands::burn::burn().await,
        Opt::Vote => commands::vote::vote().await,
        Opt::Verify => commands::verify::verify().await,
    }
}
