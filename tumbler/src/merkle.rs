use std::fs::File;

use ff_ce::{PrimeField, PrimeFieldRepr};
use poseidon_rs::{Fr, Poseidon};
use serde::Serialize;

use num_bigint::{BigInt, BigUint};
use num_traits::{FromBytes, Num};

const TREE_DEPTH: usize = 3;

pub struct MerkleTree {
    depth: usize,
    leaves: Vec<Fr>,
    hasher: Poseidon,
}

impl MerkleTree {
    pub fn new(depth: usize) -> Self {
        Self {
            depth,
            leaves: Vec::new(),
            hasher: Poseidon::new(),
        }
    }

    pub fn insert(&mut self, hex_commitment: &str) {
        let bytes = hex_commitment.strip_prefix("0x").unwrap_or(hex_commitment);

        let fr: Fr = ff_ce::from_hex(bytes).unwrap();
        self.leaves.push(fr);
    }

    pub fn contains(&self, hex_commitment: &str) -> bool {
        let bytes = hex_commitment.strip_prefix("0x").unwrap_or(hex_commitment);

        let fr: Fr = ff_ce::from_hex(bytes).unwrap();
        self.leaves.contains(&fr)
    }

    pub fn root(&self) -> BigUint {
        let mut current = self.leaves.clone();

        let size = 1 << self.depth;

        while current.len() < size {
            current.push(Fr::from_str("0").unwrap())
        }

        let mut level = current;

        for _ in 0..self.depth {
            let mut next = vec![];
            for i in (0..level.len()).step_by(2) {
                // left leaf, right leaf
                let left_leaf = level[i];
                let right_leaf = if i + 1 < level.len() {
                    level[i + 1]
                } else {
                    left_leaf
                };
                next.push(self.hasher.hash(vec![left_leaf, right_leaf]).unwrap());
            }

            level = next;
        }

        // Get the first node - if there are no nodes, return a zero hash
        BigUint::from_bytes_le(&fr_to_le_bytes(level.first().unwrap()))
    }

    pub fn generate_proof(&mut self, index: usize) -> MerkleProof {
        assert!(
            index < self.leaves.len(),
            "Index out of bounds: {} but leaves len is {}",
            index,
            self.leaves.len()
        );
        let tree = generate_tree(&self.leaves, &mut self.hasher);
        let (elements, indices) = generate_proof(index, &tree);

        MerkleProof {
            leaf: BigUint::from_bytes_le(&fr_to_le_bytes(&self.leaves[index])),
            root: self.root(),
            path_elements: elements
                .iter()
                .map(|f| BigUint::from_bytes_le(&fr_to_le_bytes(f)))
                .collect(),
            path_indices: indices,
        }
    }
}

fn fr_to_le_bytes(fr: &Fr) -> [u8; 32] {
    let repr = fr.into_repr(); // This gives you the internal BigInteger type
    let mut bytes = [0u8; 32];
    repr.write_le(&mut bytes[..]).unwrap();
    bytes
}

#[derive(Serialize)]
pub struct MerkleProof {
    #[serde(with = "biguint_as_string")]
    leaf: BigUint,
    #[serde(with = "biguint_as_string")]
    root: BigUint,
    #[serde(with = "vec_biguint_as_string")]
    path_elements: Vec<BigUint>,
    path_indices: Vec<u8>,
}

pub fn poseidon_hash(input: Vec<Fr>, hasher: &mut Poseidon) -> Fr {
    hasher.hash(input).unwrap()
}

pub fn default_leaf() -> Fr {
    Fr::from_str("0").unwrap()
}

pub fn generate_tree(leaves: &[Fr], hasher: &mut Poseidon) -> Vec<Vec<Fr>> {
    let mut layers = vec![leaves.to_vec()];

    for d in 0..TREE_DEPTH {
        let current = layers.last().unwrap();
        let mut next = vec![];

        for i in (0..current.len()).step_by(2) {
            let left = current[i];
            let right = if i + 1 < current.len() {
                current[i + 1]
            } else {
                default_leaf()
            };
            let parent = poseidon_hash(vec![left, right], hasher);
            next.push(parent);
        }

        layers.push(next);
    }

    layers
}

pub fn generate_proof(index: usize, tree: &[Vec<Fr>]) -> (Vec<Fr>, Vec<u8>) {
    let mut path_elements = vec![];
    let mut path_indices = vec![];
    let mut idx = index;

    for depth in 0..TREE_DEPTH {
        let layer = &tree[depth];

        // Ensure sibling_idx doesn't go out of bounds
        let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
        let sibling = layer.get(sibling_idx).cloned().unwrap_or(default_leaf());

        path_elements.push(sibling);
        path_indices.push((idx % 2) as u8);
        idx /= 2;
    }

    (path_elements, path_indices)
}

pub fn save_proof_as_json(proof: &MerkleProof, path: &str) {
    let json = serde_json::to_vec_pretty(&proof).unwrap();
    std::fs::write(path, json).unwrap();
}

mod biguint_as_string {
    use num_bigint::BigUint;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(x: &BigUint, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&x.to_str_radix(10))
    }

    pub fn deserialize<'de, D>(d: D) -> Result<BigUint, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        BigUint::parse_bytes(s.as_bytes(), 10)
            .ok_or_else(|| serde::de::Error::custom("Invalid BigUint"))
    }
}

mod vec_biguint_as_string {
    use num_bigint::BigUint;
    use serde::ser::{SerializeSeq, Serializer};

    pub fn serialize<S>(values: &Vec<BigUint>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(values.len()))?;
        for value in values {
            seq.serialize_element(&value.to_str_radix(10))?;
        }
        seq.end()
    }
}
