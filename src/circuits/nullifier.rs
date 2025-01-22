use std::fs::{self, File};
use primitive_types::U256;
use std::io::Write;
use log::{info, error};
use crate::circuits::Circuit;

pub struct NullifierCircuit {
    private_key: U256,
    blinding_factor: u64,
    ceremony_id: u64,
    nullifier: String,
}

impl NullifierCircuit {
    pub fn new(private_key: U256, blinding_factor: u64, ceremony_id: u64, nullifier: String) -> Self {
        Self {
            private_key,
            blinding_factor,
            ceremony_id,
            nullifier,
        }
    }
}

impl Circuit for NullifierCircuit {
    fn generate_inputs(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Generating inputs ...");
        let inputs = format!(
            "{{\"privateKey\": \"{}\",
            \"blindingFactor\": \"{}\",
            \"ceremonyID\": \"{}\",
            \"nullifier\": \"{}\" }}",
            self.private_key.to_string(),
            self.blinding_factor.to_string(),
            self.ceremony_id.to_string(),
            U256::from_str_radix(&self.nullifier[2..], 16).unwrap().to_string()
        );

        let inputs_path = "inputs/nullifier.json";
        let mut file = File::create(inputs_path)?;
        file.write_all(inputs.to_string().as_bytes())?;
        info!("Input generated.");
        Ok(())
    }

    fn generate_witness(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Generating witness ...");
        let calculate_command = "snarkjs wtns calculate circuits/nullifier/nullifier_js/nullifier.wasm inputs/nullifier.json circuits/nullifier/witness.wtns";
        let calculate_output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&calculate_command)
            .output()?;

        if !calculate_output.status.success() {
            let calculate_stdout = String::from_utf8_lossy(&calculate_output.stdout);
            println!("Standard output:\n{}", calculate_stdout);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate witness")));
        }

        let export_command = "snarkjs wtns export json circuits/nullifier/witness.wtns circuits/nullifier/witness.json";
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
        let setup_command = "snarkjs groth16 setup circuits/nullifier/nullifier.r1cs circuits/setup/pot12_final.ptau circuits/nullifier/nullifier_0000.zkey";
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

        let command = "snarkjs groth16 prove circuits/nullifier/nullifier_0000.zkey circuits/nullifier/witness.wtns circuits/proofs/nullifier_proof.json circuits/proofs/nullifier_public.json";
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
        let setup_command = "snarkjs zkey export verificationkey circuits/nullifier/nullifier_0000.zkey circuits/nullifier/verification_key.json";
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
        let command = "snarkjs groth16 verify circuits/nullifier/verification_key.json circuits/proofs/nullifier_public.json circuits/proofs/nullifier_proof.json";
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