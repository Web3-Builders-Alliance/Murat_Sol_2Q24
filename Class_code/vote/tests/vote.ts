import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vote } from "../target/types/vote";

describe("vote", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vote as Program<Vote>;

  const url = "https://wba.dev";

  const voteAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(url)],
    program.programId
  )[0];

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(url)
      .accountsPartial({
        payer: getProvider.wallet.publicKey,
        voteAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    let voteState = await program.account.voteState.fetch(voteAccount);
    console.log("\nyour vote score is", voteState.score.toString());
  });

  it("upvote", async () => {
    const tx = await program.methods
      .upvote(url)
      .accounts({
        voteAccount,
      })
      .rpc();
    console.log("your signature is", tx);

    let voteState = await program.account.voteState.fetch(voteAccount);
    console.log("\nYour vote score is", voteState.score.toString());
  });
});
