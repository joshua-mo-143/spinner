use std::{
    collections::HashSet,
    process::{Command, ExitStatus, Stdio},
};

use crate::{hash::hash_secret, merkle::MerkleTree};

pub struct Tumbler {
    pub tree: MerkleTree,
    // A list of already-used hashes. Prevents double spending by checking against this hashset first.
    used_nullifiers: HashSet<String>,
}

impl Tumbler {
    pub fn new(tree_depth: usize) -> Self {
        Self {
            tree: MerkleTree::new(tree_depth),
            used_nullifiers: HashSet::new(),
        }
    }

    pub fn deposit(&mut self, secret: &str) {
        let commitment = hash_secret(secret);
        self.tree.insert(&commitment);
        println!("Deposited commitment: {commitment}");
    }

    pub fn withdraw(&mut self, secret: &str) -> bool {
        let commitment = hash_secret(secret);
        let nullifier = format!("nullifier:{secret}");

        if self.used_nullifiers.contains(&nullifier) {
            println!("Error: Nullifier already used (double spend).");
            return false;
        }

        if !self.tree.contains(&commitment) {
            println!("Error: Commitment not found in tree");
            return false;
        }

        // Use snarkjs to verify the proof:
        let verified = verify_proof_with_snarkjs(
            "./circuit/verification_key.json",
            "./generated_proof.json",
            "./generated_public.json",
        )
        .unwrap();

        if !verified {
            println!("Error: Invalid ZK proof.");
            return false;
        }

        self.used_nullifiers.insert(nullifier);
        println!("Withdrawal successful for secret: {secret}");
        true
    }
}

pub fn verify_proof_with_snarkjs(
    verification_key_path: &str,
    proof_path: &str,
    public_inputs_path: &str,
) -> std::io::Result<bool> {
    let output = Command::new("node")
        .arg("circuit/prove.js")
        .output()
        .inspect_err(|x| println!("Proving error: {x}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{stdout}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{stderr}");

    if !stderr.is_empty() {
        eprintln!("proving stderr: {}", stderr);
    }

    let output = Command::new("snarkjs")
        .args(&[
            "groth16",
            "verify",
            verification_key_path,
            public_inputs_path,
            proof_path,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .inspect_err(|x| println!("Proving error: {x}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{stdout}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{stderr}");

    if !stderr.is_empty() {
        eprintln!("snarkjs stderr: {}", stderr);
    }

    // snarkjs outputs a line "Verification OK" or "Verification Failed"
    for line in stdout.lines() {
        if line.contains("OK!") {
            return Ok(true);
        }
        if line.contains("Failed") {
            eprintln!("Verification failed");
            return Ok(false);
        }
    }

    // If output is unexpected, treat as failure
    Ok(false)
}
