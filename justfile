# setup circuit dependencies for generating stuff from circuit
setup:
    npm i --prefix circuit

ptau:
    wget https://raw.githubusercontent.com/iden3/snarkjs/master/test/powersOfTau28_hez_final_14.ptau

# prep circuit and create all ZK stuff
prep:
    circom circuit/main.circom  --r1cs --wasm -o circuit
    npx snarkjs groth16 setup circuit/main.r1cs powersOfTau28_hez_final_18.ptau circuit/circuit_0000.zkey
    npx snarkjs zkey contribute circuit_0000.zkey circuit_final.zkey --name="Joshua Mo"
    npx snarkjs zkey export verificationkey circuit/circuit_0000.zkey circuit/verification_key.json
    npx snarkjs zkey export verificationkey circuit/circuit_final.zkey circuit/verification_key.json
    npx snarkjs zkey export solidityverifier circuit_final.zkey Verifier.sol

# prove/verify the tumbler works locally
# this additionally proves, manually modifies calldata and verifies
# this would be the equiv. of using `snarkjs groth16 prove` and `snarkjs generatecall --proof proof.json --public public.json`
prove-dev:
    cargo run

# Set up final circuit keys for prod
# keys:
#    Below not required unless you're going into prod
#    snarkjs zkey beacon circuit_0000.zkey circuit_final.zkey <random-beacon-hash> 10 --name="Final Beacon"
