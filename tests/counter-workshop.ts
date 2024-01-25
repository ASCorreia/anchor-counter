import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterWorkshop } from "../target/types/counter_workshop";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { get } from "http";

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
const getMetadata = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
  return (
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )
  )[0];
};

describe("counter-workshop", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterWorkshop as Program<CounterWorkshop>;

  const user = Keypair.generate();
  console.log("User pubkey = ", user.publicKey.toBase58());

  const userPDA = findProgramAddressSync([Buffer.from("counter"), provider.publicKey.toBuffer()], program.programId)[0];

  it("Initialize Counter Account!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      user: provider.publicKey,
      counter: user.publicKey,
      systemProgram: SystemProgram.programId
    })
    .signers([user])
    .rpc(
      {skipPreflight: true}
    );
    console.log("\n\nYour transaction signature", tx);
    console.log("Counter value = ", (await program.account.counter.fetch(user.publicKey)).counter);
  });

  it("Increment Counter", async() => {
    const tx = await program.methods.increment().accounts({
      counter: user.publicKey
    })
    .rpc();

    console.log("\n\nYour transaction signature", tx);
    console.log("Counter value = ", (await program.account.counter.fetch(user.publicKey)).counter);
  })

  it("Decrement Counter", async() => {
    const tx = await program.methods.decrement().accounts({
      counter: user.publicKey
    })
    .rpc();

    console.log("\n\nYour transaction signature", tx);
    console.log("Counter value = ", (await program.account.counter.fetch(user.publicKey)).counter);
  })

  it("Initialize PDA Counter", async() => {
    const tx = await program.methods.initializePda().accounts({
      user: provider.publicKey,
      counterPda: userPDA,
    })
    .rpc();

    console.log("\n\nYour transaction signature", tx);
    console.log("Counter value = ", (await program.account.counterPda.fetch(userPDA)).counter);
  })

  it("Increment PDA Counter", async() => {
    const tx = await program.methods.incrementPda().accounts({
      user: provider.publicKey,
      counterPda: userPDA,
    })
    .rpc();

    console.log("\n\nYour transaction signature", tx);
    console.log("Counter value = ", (await program.account.counterPda.fetch(userPDA)).counter);
  });

  xit("Mint Spl Tokens", async() => {
    const mint = Keypair.generate();
    const metadata = await getMetadata(mint.publicKey);
    const userAta = getAssociatedTokenAddressSync(mint.publicKey, provider.publicKey);

    const tx = await program.methods.mintSpl().accounts({
      user: provider.publicKey,
      mint: mint.publicKey,
      userAta: userAta,
      metadata: metadata,
      counterPda: userPDA,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([mint])
    .rpc({
      skipPreflight: true
    })
    console.log("\n\nYour transaction signature", tx);
  });

  it("Decrement PDA Counter", async() => {
    const tx = await program.methods.decrementPda().accounts({
      user: provider.publicKey,
      counterPda: userPDA,
    })
    .rpc();

    console.log("\n\nYour transaction signature", tx);
    console.log("Counter value = ", (await program.account.counterPda.fetch(userPDA)).counter);
  });
});
