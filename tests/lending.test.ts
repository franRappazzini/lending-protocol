import * as anchor from "@coral-xyz/anchor";

import { getDepositContext, getLendingTokens } from "@jup-ag/lend/dist/earn";

import { Lending } from "../target/types/lending";
import PUBKEYS from "./lib/pubkeys";
import { Program } from "@coral-xyz/anchor";
import { bn } from "./lib/functions";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

describe("lending", () => {
  const provider = anchor.AnchorProvider.env();
  const { connection, wallet } = provider;

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.lending as Program<Lending>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Deposit!", async () => {
    const allTokens = await getLendingTokens({ connection });

    return console.log({ allTokens });

    const depositContext = await getDepositContext({
      asset: PUBKEYS.DEV.USDC,
      signer: wallet.publicKey,
      connection,
    });

    console.log({ depositContext });

    const amount = 1_000_000; // 1 USDC

    const walletAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      PUBKEYS.DEV.USDC,
      wallet.publicKey
    );
    console.log("User USDC ATA:", walletAta.address.toBase58());

    const ix = await program.methods
      .deposit(bn(amount))
      .accounts({
        // Your program accounts
        // userAccount: userAccount,

        // Jupiter Lend accounts (from context)
        // signer: depositContext.signer,
        depositorTokenAccount: depositContext.depositorTokenAccount,
        recipientTokenAccount: depositContext.recipientTokenAccount,
        lendingAdmin: depositContext.lendingAdmin,
        lending: depositContext.lending,
        fTokenMint: depositContext.fTokenMint,
        mint: depositContext.mint,
        supplyTokenReservesLiquidity: depositContext.supplyTokenReservesLiquidity,
        lendingSupplyPositionOnLiquidity: depositContext.lendingSupplyPositionOnLiquidity,
        rateModel: depositContext.rateModel,
        vault: depositContext.vault,
        liquidity: depositContext.liquidity,
        liquidityProgram: depositContext.liquidityProgram,
        rewardsRateModel: depositContext.rewardsRateModel,
        tokenProgram: depositContext.tokenProgram,
        // ... all other accounts from context
        // lendingProgram: PUBKEYS.DEV.LENDING_PROGRAM,
      })
      .instruction();

    const buildTx = new anchor.web3.Transaction().add(ix);

    const tx = await anchor.web3.sendAndConfirmTransaction(connection, buildTx, [wallet.payer], {
      skipPreflight: true,
    });

    console.log("Transaction successful:", tx);
  });
});
