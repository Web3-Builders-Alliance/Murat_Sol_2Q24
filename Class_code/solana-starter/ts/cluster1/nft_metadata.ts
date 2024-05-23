import wallet from "./wallet/wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
    const image =
      "https://arweave.net/pocZJo5YACV4ETeFg5DG9v3sB7C4mf7oMvtCQ5_Fiq4";
    const metadata = {
      name: "massive carpet",
      symbol: "MC",
      description: "what a massive carpet it is",
      image,
      attributes: [
        { trait_type: "color", value: "green" },
        { trait_type: "rarity", value: "1" },
      ],
      properties: {
        files: [
          {
            type: "image/png",
            uri: image,
          },
        ],
      },
    };
    const myUri = await bundlrUploader.uploadJson(metadata);
    console.log(`image uri is: ${myUri}`);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
