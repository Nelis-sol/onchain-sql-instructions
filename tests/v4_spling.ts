import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { V4Spling } from "../target/types/v4_spling";
import { PublicKey, ComputeBudgetProgram, Transaction } from '@solana/web3.js'

describe("v4_spling", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.V4Spling as Program<V4Spling>;

  let random_pubkey = anchor.web3.Keypair.generate();

  it("tx 1 sent", async () => {

    const [SenderLogPDA] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('sender'),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )


    let tx = await program.methods
    .submitTransaction(1, 1, random_pubkey.publicKey, "example hash", null, "new nonce")
    .accounts({
      sender: provider.wallet.publicKey,
      senderLog: SenderLogPDA,
    })
    .rpc()

    console.log("Your transaction signature", tx);

  });


  it("tx2 sent", async () => {

    const [SenderLogPDA] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('sender'),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )


    let tx = await program.methods
    .submitTransaction(1, 1, random_pubkey.publicKey, "example hash", null, null)
    .accounts({
      sender: provider.wallet.publicKey,
      senderLog: SenderLogPDA,
    })
    .rpc()

    console.log("Your transaction signature", tx);

  });

  it("tx3 sent", async () => {

    const [SenderLogPDA] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('sender'),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )


    let tx = await program.methods
    .submitTransaction(1, 1, random_pubkey.publicKey, "example hash", null, "nonce")
    .accounts({
      sender: provider.wallet.publicKey,
      senderLog: SenderLogPDA,
    })
    .rpc()

    console.log("Your transaction signature", tx);

  });


  it("New operation created", async () => {

    const [OperationPDA] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('operation'),
        anchor.utils.bytes.utf8.encode('INSERT'),
      ],
      program.programId
    )


    let tx = await program.methods
    .createOperation(1, "INSERT", "This is operation inserts a new value")
    .accounts({
      authority: provider.wallet.publicKey,
      operation: OperationPDA,
    })
    .rpc()

    const opp = await program.account.operation.fetch(OperationPDA);
    console.log(opp);

  });


  it("New schema is created", async () => {

    const [SchemaPDA] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('schema'),
        anchor.utils.bytes.utf8.encode('APP'),
      ],
      program.programId
    )


    let tx = await program.methods
    .createSchema(1, "APP", "This is an app object.")
    .accounts({
      authority: provider.wallet.publicKey,
      schema: SchemaPDA,
    })
    .rpc()

    const schema = await program.account.schema.fetch(SchemaPDA);
    console.log(schema);

  });


});
