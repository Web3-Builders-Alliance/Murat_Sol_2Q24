import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  percentAmount,
} from "@metaplex-foundation/umi";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";

import wallet from "./wallet/wba-wallet.json";
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
  try {
    const uri =
      "https://arweave.net/Bwv5G10XzlHxZ2Cma32ENi8Sw1E9aXKQl1FWITNOBd8";
    let tx = createNft(umi, {
      mint,
      name: "massive carpet",
      symbol: "MC",
      uri,
      sellerFeeBasisPoints: percentAmount(4),
    });
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    console.log(
      `succesfully minted, tx is here:https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
    console.log("Mint Address: ", mint.publicKey);
  } catch (error) {
    // Log an error message if something goes wrong
    console.log(`sth went wrong: ${error}`);
  }
})();
