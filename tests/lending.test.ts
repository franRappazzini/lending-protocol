import * as anchor from "@coral-xyz/anchor";

import { Lending } from "../target/types/lending";
import PUBKEYS from "./lib/pubkeys";
import { Program } from "@coral-xyz/anchor";
import { bn } from "./lib/functions";
import { getDepositContext } from "@jup-ag/lend/dist/earn";

// import { getDepositContext } from "@jup-ag/lend";

describe("lending", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet;

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.lending as Program<Lending>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Deposit!", async () => {
    const depositContext = await getDepositContext({
      asset: PUBKEYS.DEV.USDC,
      signer: wallet.publicKey,
      connection,
    });

    console.log({ depositContext });

    const amount = 10_000_000; // 10 USDC

    const tx = await program.methods
      .deposit(bn(amount))
      .accounts({
        // Your program accounts
        userAccount: userAccount,

        // Jupiter Lend accounts (from context)
        signer: depositContext.signer,
        depositorTokenAccount: depositContext.depositorTokenAccount,
        recipientTokenAccount: depositContext.recipientTokenAccount,
        lendingAdmin: depositContext.lendingAdmin,
        lending: depositContext.lending,
        fTokenMint: depositContext.fTokenMint,
        // ... all other accounts from context
        lendingProgram: PUBKEYS.DEV.LENDING_PROGRAM,
      })
      .rpc({ skipPreflight: true });

    console.log("Transaction successful:", tx);
  });
});
