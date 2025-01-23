use structopt::StructOpt;
mod circuits;
mod commands;
mod utils;
use commands::burn::Burn;
use commands::burn_address::BurnAddress;
use commands::nullifier::Nullifier;
use env_logger::Env;

#[derive(Debug, StructOpt)]
enum Opt {
    BurnAddress(BurnAddress),
    Burn(Burn),
    Nullifier(Nullifier),
    Vote,
    Verify,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opt = Opt::from_args();
    match opt {
        Opt::BurnAddress(burn_address) => {
            commands::burn_address::burn_address(burn_address).await;
        }
        Opt::Burn(burn_data) => {
            commands::burn::burn(burn_data).await;
        }
        Opt::Nullifier(nullifier) => {
            commands::nullifier::generate_nullifier(nullifier);
        }
        Opt::Vote => {
            commands::vote::vote().await;
        }
        Opt::Verify => {
            commands::verify::verify().await;
        }
    }
}
