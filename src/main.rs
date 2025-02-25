use structopt::StructOpt;
mod circuits;
mod commands;
mod utils;
use commands::burn::Burn;
use commands::burn_address::BurnAddress;
use commands::nullifier::Nullifier;
use commands::merkle_tree::UserIndex;
use env_logger::Env;

#[derive(Debug, StructOpt)]
enum Opt {
    BurnAddress(BurnAddress),
    Burn(Burn),
    Nullifier(Nullifier),
    Vote,
    Verify,
    GenerateTree,
    GenerateProof(UserIndex)
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
        Opt::GenerateTree => {
            commands::merkle_tree::generate_tree().await;
        }
        Opt::GenerateProof(user_index)=> {
            let tree = commands::merkle_tree::generate_tree().await;
            commands::merkle_tree::generate_proof(tree, user_index.index).await;
        }
        Opt::Verify => {
            commands::verify::verify().await;
        }
    }
}
