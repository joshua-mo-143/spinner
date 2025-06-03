use spinner::{
    merkle::{MerkleTree, save_proof_as_json},
    tumbler::Tumbler,
};

fn main() {
    // let mut mixer = Tumbler::new(3);

    // mixer.deposit("mysecret123");
    // mixer.deposit("anotheruser");

    // println!("Merkle Root: {}", mixer.tree.root());

    // mixer.withdraw("mysecret123");
    // assert!(!mixer.withdraw("mysecret123")); // should fail
    // assert!(!mixer.withdraw("unknown")); // should fail
    // let hex_commitment1 = hex::encode("secret1");
    // let hex_commitment2 = hex::encode("secret2");

    // let mut tree = MerkleTree::new(3);
    // tree.insert(&hex_commitment1);
    // tree.insert(&hex_commitment2);
    // let proof = tree.generate_proof(0);

    // save_proof_as_json(&proof, "proof_inputs.json");

    let mut tumbler = Tumbler::new(3);

    tumbler.deposit("foo");
    tumbler.deposit("bar");

    tumbler.withdraw("foo");
}
