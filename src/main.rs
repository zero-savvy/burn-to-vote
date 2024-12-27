use structopt::StructOpt;
mod commands;
use commands::burn::Burn;
use commands::burn_address::BurnAddress;

#[derive(Debug, StructOpt)]
enum Opt {
    BurnAddress(BurnAddress),
    Burn(Burn),
    Vote,
    Verify,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::BurnAddress(burn_address) => {
            commands::burn_address::burn_address(burn_address).await;
        }
        Opt::Burn(burn_data) => {
            commands::burn::burn(burn_data).await;
        }
        Opt::Vote => {
            commands::vote::vote().await;
        }
        Opt::Verify => {
            commands::verify::verify().await;
        }
    }
}
// cargo run -- Burn --private-key <private_key> --ceremony-id <ceremony_id> --blinding-factor <blinding_factor> --personal-id <personal_id> --vote <vote>
