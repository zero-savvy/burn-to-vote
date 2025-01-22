use std::fs::{self, File};
use primitive_types::U256;
use std::io::Write;
use log::{info, error};
use crate::circuits::Circuit;

pub struct BurnAddressCircuit {
    address: String,
    private_key: U256,
    blinding_factor: u64,
    ceremony_id: u64,
    personal_id: u64,
    vote: u64,
}

impl BurnAddressCircuit {
    pub fn new(address: String, private_key: U256, blinding_factor: u64, ceremony_id: u64, personal_id: u64, vote: u64) -> Self {
        Self {
            address,
            private_key,
            blinding_factor,
            ceremony_id,
            personal_id,
            vote,
        }
    }
}

impl Circuit for BurnAddressCircuit {
    fn generate_inputs(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Generating inputs ...");
        let inputs = format!(
            "{{ \"address\": \"{}\",
            \"privateKey\": \"{}\",
            \"blinding_factor\": \"{}\",
            \"ceremonyID\": \"{}\",
            \"personalID\": \"{}\",
            \"vote\": \"{}\" }}",
            U256::from_str_radix(&self.address[2..], 16).unwrap().to_string(),
            self.private_key.to_string(),
            serde_json::to_string(&self.blinding_factor).unwrap(),
            serde_json::to_string(&self.ceremony_id).unwrap(),
            serde_json::to_string(&self.personal_id).unwrap(),
            serde_json::to_string(&self.vote).unwrap()
        );

        let inputs_path = "inputs/burn_address.json";
        let mut file = File::create(inputs_path)?;
        file.write_all(inputs.to_string().as_bytes())?;
        info!("Input generated.");
        Ok(())
    }

    fn generate_witness(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Generating witness ...");
        let calculate_command = "snarkjs wtns calculate circuits/burnAddress_files/burnAddress_js/burnAddress.wasm inputs/burn_address.json circuits/burnAddress_files/witness.wtns";
        let calculate_output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&calculate_command)
            .output()?;

        if !calculate_output.status.success() {
            let calculate_stdout = String::from_utf8_lossy(&calculate_output.stdout);
            println!("Standard output:\n{}", calculate_stdout);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate witness")));
        }

        let export_command = "snarkjs wtns export json circuits/burnAddress_files/witness.wtns circuits/burnAddress_files/witness.json";
        let export_output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&export_command)
            .output()?;

        if !export_output.status.success() {
            let export_stdout = String::from_utf8_lossy(&export_output.stdout);
            println!("Standard output:\n{}", export_stdout);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to export witness")));
        }

        info!("Witness Generated.");
        Ok(())
    }

    fn setup_zkey(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Setting up zkey ...");
        let setup_command = "snarkjs groth16 setup circuits/burnAddress_files/burnAddress.r1cs circuits/setup/pot12_final.ptau circuits/burnAddress_files/burnAddress_0000.zkey";
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(setup_command)
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Standard output:\n{}", stdout);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to setup zkey")));
        }

        info!("Zkey Generated.");
        Ok(())
    }

    fn generate_proof(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Generating proof ...");
        fs::create_dir_all("circuits/proofs")?;

        let command = "snarkjs groth16 prove circuits/burnAddress_files/burnAddress_0000.zkey circuits/burnAddress_files/witness.wtns circuits/proofs/burnAddress_proof.json circuits/proofs/burnAddress_public.json";
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Standard output:\n{}", stdout);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate proof")));
        }

        info!("Proof Generated.");

        Ok(())
    }

    fn setup_vkey(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Setting up vkey ...");
        let setup_command = "snarkjs zkey export verificationkey circuits/burnAddress_files/burnAddress_0000.zkey circuits/burnAddress_files/verification_key.json";
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&setup_command)
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Standard output:\n{}", stdout);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to setup vkey")));
        }

        info!("Vkey Generated.");
        Ok(())
    }
    
    fn verify_proof(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Verifying proof ...");
        let command = "snarkjs groth16 verify circuits/burnAddress_files/verification_key.json circuits/proofs/burnAddress_public.json circuits/proofs/burnAddress_proof.json";
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Standard output:\n{}", stdout);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to verify proof")));
        }

        info!("PROOF VERIFIED.");
        Ok(())
    }
}