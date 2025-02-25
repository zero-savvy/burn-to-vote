pub mod burn_address_c;
pub mod merkle_tree_c;
pub mod nullifier_c;
use log::info;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub trait Circuit {
    fn generate_input_file(&self, inputs: String) -> Result<(), Box<dyn Error>>;
    fn generate_witness(&self) -> Result<(), Box<dyn Error>>;
    fn setup_zkey(&self) -> Result<(), Box<dyn Error>>;
    fn setup_vkey(&self) -> Result<(), Box<dyn Error>>;
    fn generate_proof(&self) -> Result<(), Box<dyn Error>>;
    fn verify_proof(&self) -> Result<(), Box<dyn Error>>;
}

pub struct CircuitIdentifier<'a> {
    pub circuit_name: &'a str,
}

impl<'a> Circuit for CircuitIdentifier<'a> {
    fn generate_input_file(&self, inputs: String) -> Result<(), Box<dyn Error>> {
        info!("Generating inputs ...");
        fs::create_dir_all("circuits/inputs")?;
        let inputs_path = format!("circuits/inputs/{}.json", self.circuit_name);
        let mut file = File::create(inputs_path)?;
        file.write_all(inputs.as_bytes())?;
        info!("Inputs generated.");
        Ok(())
    }

    fn generate_witness(&self) -> Result<(), Box<dyn Error>> {
        info!("Generating witness ...");
        let calculate_command = format!(
            "snarkjs wtns calculate circuits/{name}/{name}_js/{name}.wasm circuits/inputs/{name}.json circuits/{name}/witness.wtns",
            name = self.circuit_name
        );
        self.run_command(&calculate_command)?;

        let export_command = format!(
            "snarkjs wtns export json circuits/{name}/witness.wtns circuits/{name}/witness.json",
            name = self.circuit_name
        );
        self.run_command(&export_command)?;

        info!("Witness generated.");
        Ok(())
    }

    fn setup_zkey(&self) -> Result<(), Box<dyn Error>> {
        info!("Setting up zkey ...");

        let setup_command = format!(
            "snarkjs groth16 setup circuits/{name}/{name}.r1cs circuits/setup/pot12_final.ptau circuits/{name}/{name}_0000.zkey",
            name = self.circuit_name
        );
        self.run_command(&setup_command)?;

        info!("Zkey Generated.");
        Ok(())
    }

    fn setup_vkey(&self) -> Result<(), Box<dyn Error>> {
        info!("Setting up vkey ...");

        let vkey_command = format!(
            "snarkjs zkey export verificationkey circuits/{name}/{name}_0000.zkey circuits/{name}/verification_key.json",
            name = self.circuit_name
        );
        self.run_command(&vkey_command)?;

        info!("Vkey Generated.");
        Ok(())
    }

    fn generate_proof(&self) -> Result<(), Box<dyn Error>> {
        info!("Generating proof ...");

        fs::create_dir_all("circuits/proofs")?;

        let proof_command = format!(
            "snarkjs groth16 prove circuits/{name}/{name}_0000.zkey circuits/{name}/witness.wtns circuits/proofs/{name}_proof.json circuits/proofs/{name}_public.json",
            name = self.circuit_name
        );
        self.run_command(&proof_command)?;

        info!("Proof generated.");
        Ok(())
    }

    fn verify_proof(&self) -> Result<(), Box<dyn Error>> {
        info!("Verifying proof ...");

        let verify_command = format!(
            "snarkjs groth16 verify circuits/{name}/verification_key.json circuits/proofs/{name}_public.json circuits/proofs/{name}_proof.json",
            name = self.circuit_name
        );
        self.run_command(&verify_command)?;

        info!("Proof verified.");
        Ok(())
    }
}

impl<'a> CircuitIdentifier<'a> {
    fn run_command(&self, command: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new("sh").arg("-c").arg(command).output()?;
        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command failed: {}", stdout),
            )));
        }
        Ok(())
    }
}
