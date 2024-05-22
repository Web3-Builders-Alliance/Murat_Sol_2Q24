import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "./wallet/wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("27ftsXoAMcVpRMCPFt3CpdrqPSADRi2Td9UU2rL24MpA");

// Recipient address
const to = new PublicKey("C9ZZ4ggdMceMGPZjuseTapyBR2n7WMFfgLiVBwBd7hFV");

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const from_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    // Get the token account of the toWallet address, and if it does not exist, create it
    const to_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );
    // Transfer the new token to the "toTokenAccount" we just created
    const tx = await transfer(
      connection,
      keypair,
      from_ata.address,
      to_ata.address,
      keypair.publicKey,
      1000n
    );
    // log if success
    console.log(
      `Succesfully Minted!. Transaction Here: https://explorer.solana.com/tx/${tx}?cluster=devnet`
    );
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
