import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { createAssociatedTokenAccount, createMint, getAccount, mintTo } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";

describe("amm", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.amm as Program<Amm>;

  let token_a_mint;
  let token_b_mint;
  let lp_mint;
  let pool_account, bump;

  let user_a_ata
  let user_b_ata
  let user_lp_ata

  it("Is initialized!", async () => {
    token_a_mint = await createMint(connection, provider.wallet.payer, provider.publicKey, null, 6);
    token_b_mint = await createMint(connection, provider.wallet.payer, provider.publicKey, null, 6);

    [pool_account, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), token_a_mint.toBuffer(), token_b_mint.toBuffer()],
      program.programId
    );

    let [lp_mint_1, lp_bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("mint"), pool_account.toBuffer()],
      program.programId
    );

    lp_mint = lp_mint_1;



    const tx = await program.methods.initialize().accounts({
      tokenAMint: token_a_mint.toBase58(),
      tokenBMint: token_b_mint.toBase58(),
    }).rpc();

    // const poolAccountData = await program.account.pool.fetch(pool_account);
    // console.log("Pool account data:", poolAccountData);

    console.log("Your transaction signature", tx);
  });

  it("Providing LP!", async () => {
    user_a_ata = await createAssociatedTokenAccount(connection, provider.wallet.payer, token_a_mint, provider.publicKey)
    user_b_ata = await createAssociatedTokenAccount(connection, provider.wallet.payer, token_b_mint, provider.publicKey)
    user_lp_ata = await createAssociatedTokenAccount(connection, provider.wallet.payer, lp_mint, provider.publicKey)

    await mintTo(
      connection,
      provider.wallet.payer,
      token_a_mint,
      user_a_ata,
      provider.publicKey,
      BigInt(1000_000000)
    )

    await mintTo(
      connection,
      provider.wallet.payer,
      token_b_mint,
      user_b_ata,
      provider.publicKey,
      BigInt(1000_000000)
    )

    const tx = await program.methods.addLiquidity(new anchor.BN(50_000_000), new anchor.BN(500_000_000)).accounts({
      // user: provider.publicKey.toBase58(),
      //@ts-ignore
      tokenAMint: token_a_mint.toBase58(),
      tokenBMint: token_b_mint.toBase58(),
    }).rpc();

    let poolAccountData = await program.account.pool.fetch(pool_account);

    poolAccountData = await program.account.pool.fetch(pool_account);
    console.log("Lp token liquidity (BN):", poolAccountData.totalLiquidty.toNumber());

    const balanceInfo1 = await connection.getTokenAccountBalance(user_a_ata);
    console.log("User A ATA Balance:", balanceInfo1.value.uiAmount);

    const balanceInfo = await connection.getTokenAccountBalance(user_b_ata);
    console.log("User B ATA Balance:", balanceInfo.value.uiAmount);

    const balanceInfo_lp = await connection.getTokenAccountBalance(user_lp_ata);
    console.log("User lp ATA Balance:", balanceInfo_lp.value.uiAmount);
  });

  

  it("Swap AMM", async () => {
    console.log("-".repeat(10));
    const tx = await program.methods.swap( true, new anchor.BN(50_000_000)).accounts({
      // user: provider.publicKey.toBase58(),
      tokenAMint: token_a_mint.toBase58(),
      tokenBMint: token_b_mint.toBase58(),
    }).rpc();

    let poolAccountData = await program.account.pool.fetch(pool_account);

    console.log("Lp token liquidity (BN):", poolAccountData.totalLiquidty.toNumber());

    const balanceInfo1 = await connection.getTokenAccountBalance(user_a_ata);
    console.log("User A ATA Balance:", balanceInfo1.value.uiAmount);

    const balanceInfo = await connection.getTokenAccountBalance(user_b_ata);
    console.log("User B ATA Balance:", balanceInfo.value.uiAmount);

  });

  it("Removing LP!", async () => {
    console.log("-".repeat(150));
    const tx = await program.methods.removeLiquidity(new anchor.BN(158_113_883)).accounts({
      // user: provider.publicKey.toBase58(),
      tokenAMint: token_a_mint.toBase58(),
      tokenBMint: token_b_mint.toBase58(),
    }).rpc();

    let poolAccountData = await program.account.pool.fetch(pool_account);

    console.log("Lp token liquidity (BN):", poolAccountData.totalLiquidty.toNumber());

    const balanceInfo1 = await connection.getTokenAccountBalance(user_a_ata);
    console.log("User A ATA Balance:", balanceInfo1.value.uiAmount);

    const balanceInfo = await connection.getTokenAccountBalance(user_b_ata);
    console.log("User B ATA Balance:", balanceInfo.value.uiAmount);

    const balanceInfo_lp = await connection.getTokenAccountBalance(user_lp_ata);
    console.log("User lp ATA Balance:", balanceInfo_lp.value.uiAmount);
  });

});
