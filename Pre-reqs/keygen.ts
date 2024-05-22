import { Keypair } from "@solana/web3.js";

let kp = Keypair.generate();
console.log(
  `you have generated a new solana wallet: ${kp.publicKey.toBase58()} secret key is: [${
    kp.secretKey
  }]`
);
