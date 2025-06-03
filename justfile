# setup circuit dependencies for generating stuff from circuit
setup:
    npm i --prefix circuit

ptau:
    wget https://raw.githubusercontent.com/iden3/snarkjs/master/test/powersOfTau28_hez_final_14.ptau

# prep circuit and create all ZK stuff
prep:
    circom circuit/main.circom  --r1cs --wasm -o circuit
    npx snarkjs groth16 setup circuit/main.r1cs powersOfTau28_hez_final_18.ptau circuit/circuit_0000.zkey
    npx snarkjs zkey export verificationkey circuit/circuit_0000.zkey circuit/verification_key.json

prove:
    cargo run
    node circuit/prove.js
    snarkjs groth16 verify circuit/verification_key.json generated_public.json generated_proof.json
