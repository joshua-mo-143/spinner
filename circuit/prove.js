const snarkjs = require("snarkjs");
const fs = require("fs");

async function generateProof() {
  let obj = JSON.parse(fs.readFileSync("proof_inputs.json"));

  console.log(obj);

  const { proof, publicSignals } = await snarkjs.groth16.fullProve(
    {
      leaf: obj["leaf"],
      root: obj["root"],
      pathElements: obj["path_elements"],
      pathIndices: obj["path_indices"],
    },
    "circuit/main_js/main.wasm",
    "circuit/circuit_0000.zkey",
  );

  // NOTE: Technically, these can be stored as .json files.
  console.log(publicSignals);
  console.log(proof);
  fs.writeFileSync("generated_proof.json", JSON.stringify(proof, null, 2));
  fs.writeFileSync(
    "generated_public.json",
    JSON.stringify(publicSignals, null, 2),
  );
}

generateProof().then(() => {
  process.exit(0);
});
