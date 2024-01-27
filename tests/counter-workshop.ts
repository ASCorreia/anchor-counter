import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterWorkshop } from "../target/types/counter_workshop";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress, getAssociatedTokenAddressSync } from "@solana/spl-token";
import * as spl from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID, associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { get } from "http";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";

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

  const mint = Keypair.generate();
  let metadata: PublicKey;
  const userAta = getAssociatedTokenAddressSync(mint.publicKey, provider.publicKey);

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

  it("Mint Spl Tokens", async() => {
    metadata = await getMetadata(mint.publicKey);

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
    })
    .signers([mint])
    .rpc({
      skipPreflight: true
    })
    console.log("\n\nYour transaction signature", tx);
    let ata_data = await provider.connection.getAccountInfo(userAta);
    console.log("Owner = ", ata_data.owner.toBase58());
    console.log("Owner of ata ", spl.AccountLayout.decode(Buffer.from(ata_data.data)).owner.toBase58());
  });

  it("Close ATA", async() => {
    const tx2 = await program.methods.closeAta().accounts({
      user: provider.publicKey,
      mint: mint.publicKey,
      userAta: userAta,
      metadata: metadata,
      counterPda: userPDA,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([mint])
    .rpc({
      skipPreflight: true
    })
    console.log("\n\nYour transaction signature", tx2);
  })

  it("Decrement PDA Counter", async() => {
    const tx = await program.methods.decrementPda().accounts({
      user: provider.publicKey,
      counterPda: userPDA,
    })
    .rpc();

    console.log("\n\nYour transaction signature", tx);
    console.log("Counter value = ", (await program.account.counterPda.fetch(userPDA)).counter);
  });

  it("Close PDA Counter", async() => {
    const tx = await program.methods.closeState().accounts({
      user: provider.publicKey,
      counterPda: userPDA,
    })
    .rpc();

    console.log("\n\nYour transaction signature", tx);
  });
});
