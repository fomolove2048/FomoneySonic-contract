import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import { Ctx, createCtx } from "./helpers/ctx";
import { CheckCtx } from "./helpers/check";
import { sleep } from "./helpers/helper";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddressSync, getTokenMetadata } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { assert, expect } from "chai";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { FomoloveSonic } from "../target/types/fomolove_sonic";

const MEME_TEAM_URL = 'https://bafybeidlf73itmw6hzskpy7amdcjzww3umwmvlwiqubbs2mkll2tnv7ojq.ipfs.nftstorage.link/me';
const CHAIN_TEAM_URL = 'https://bafybeiferm3u2nsdnzcf25ubqdy3qjbn3bu6meeeue52lrkvlr3llqbpce.ipfs.nftstorage.link/chain';

describe("fomolove-sonic", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FomoloveSonic as Program<FomoloveSonic>;
  const connection = new Connection("http://localhost:8899", 'confirmed');

  let ctx: Ctx;

  it("Is initialized!", async () => {
    ctx = await createCtx(connection, program);
    
    const tx = await program.methods.initialize(CHAIN_TEAM_URL, MEME_TEAM_URL).accountsPartial({
      maintainer: ctx.maintainer.publicKey,
      configAccount: ctx.configAccount,
      chainTeamAccount: ctx.chainTeamAccount,
      memeTeamAccount: ctx.memeTeamAccount,
      winnerAccount: ctx.winnerAccount
    }).signers([ctx.maintainer]).rpc({skipPreflight: true});

    const configAccount = await CheckCtx.config(ctx);

    const currentSeasonId = configAccount.currentSeasonId;

    const seasonIdBuffer = Buffer.from([currentSeasonId + 1]);

    ctx.seasonAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("season"), seasonIdBuffer],
      program.programId
    )[0];

    ctx.user1SeasonAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("user_season"), ctx.user1.publicKey.toBuffer(), seasonIdBuffer],
      program.programId
    )[0];
  });

  it("Create season!", async () => {
    await sleep(3000);
     const startTime = new anchor.BN(Date.now() / 1000);

    const tx = await program.methods.startSeason(startTime).accountsPartial({
      maintainer: ctx.maintainer.publicKey,
      configAccount: ctx.configAccount,
      seasonAccount: ctx.seasonAccount,
    }).signers([ctx.maintainer]).rpc();
  });

  it("Choose team!", async () => {
    const tx = await program.methods.chooseTeam({ memeTeam: {} }).accountsPartial({
      user: ctx.user1.publicKey,
      userAccount: ctx.user1Account,
      teamMemeAccount: ctx.memeTeamAccount,
      teamChainAccount: ctx.chainTeamAccount
    }).signers([ctx.user1]).rpc();
  });

  it("Can not choose team if already chosen!", async () => {
    try {
       await program.methods.chooseTeam({ chainTeam: {} }).accountsPartial({
        user: ctx.user1.publicKey,
        userAccount: ctx.user1Account,
        teamMemeAccount: ctx.memeTeamAccount,
        teamChainAccount: ctx.chainTeamAccount
      }).signers([ctx.user1]).rpc();
    } catch (error) {
      assert.isTrue(error instanceof anchor.AnchorError);
      const err: anchor.AnchorError = error;
      expect(err.error.errorCode.code).to.equal('UserAlreadyOnTeam')
    }
   
  });

  it("Register game!", async () => {

    const destinationTokenAccount = getAssociatedTokenAddressSync(
      ctx.nftMint.publicKey,
      ctx.user1.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    ctx.nftTokenAccount = destinationTokenAccount;

    const tx = await program.methods.registerGame().accountsPartial({
      user: ctx.user1.publicKey,
      userAccount: ctx.user1Account,
      userSeasonAccount: ctx.user1SeasonAccount,
      gameAccount: ctx.gameAccount,
      seasonAccount: ctx.seasonAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      nftMint: ctx.nftMint.publicKey,
      configAccount: ctx.configAccount,
      tokenAccount: destinationTokenAccount,
      systemProgram: anchor.web3.SystemProgram.programId,

    }).signers([ctx.user1, ctx.nftMint]).rpc();
  });

  it("CANNOT Make MOVE if not owner of token", async () => {
    try {
      await program.methods.makeMove({ up: {} }).accountsPartial({
        user: ctx.user2.publicKey,
        configAccount: ctx.configAccount,
        game: ctx.gameAccount,
        userAccount: ctx.user1Account,
        userTeamAccount: ctx.memeTeamAccount,
        userSeasonAccount: ctx.user1SeasonAccount,
        nftMint: ctx.nftMint.publicKey,
        nftTokenAccount: ctx.nftTokenAccount,
        winnerAccount: ctx.winnerAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID
      }).signers([ctx.user2]).rpc();
    } catch (error) {
      assert.isTrue(error instanceof anchor.AnchorError);
      const err: anchor.AnchorError = error;
      expect(err.error.errorMessage).to.equal('A token owner constraint was violated')
    }
  });

  it("Make move: UP!", async () => {
    await program.methods.makeMove({ up: {} }).accountsPartial({
      user: ctx.user1.publicKey,
      configAccount: ctx.configAccount,
      game: ctx.gameAccount,
      userAccount: ctx.user1Account,
      userTeamAccount: ctx.memeTeamAccount,
      userSeasonAccount: ctx.user1SeasonAccount,
      nftMint: ctx.nftMint.publicKey,
      nftTokenAccount: ctx.nftTokenAccount,
      winnerAccount: ctx.winnerAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID

    }).signers([ctx.user1]).rpc();
    await CheckCtx.getGameState(ctx);
  });

  it("Make move: UP!", async () => {
    await program.methods.makeMove({ up: {} }).accountsPartial({
      user: ctx.user1.publicKey,
      configAccount: ctx.configAccount,
      game: ctx.gameAccount,
      userAccount: ctx.user1Account,
      userTeamAccount: ctx.memeTeamAccount,
      userSeasonAccount: ctx.user1SeasonAccount,
      nftMint: ctx.nftMint.publicKey,
      nftTokenAccount: ctx.nftTokenAccount,
      winnerAccount: ctx.winnerAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID
    }).signers([ctx.user1]).rpc();

    await CheckCtx.getGameState(ctx);
  });

  it("Make move: LEFT!", async () => {
    await program.methods.makeMove({ left: {} }).accountsPartial({
      user: ctx.user1.publicKey,
      configAccount: ctx.configAccount,
      game: ctx.gameAccount,
      userAccount: ctx.user1Account,
      userTeamAccount: ctx.memeTeamAccount,
      userSeasonAccount: ctx.user1SeasonAccount,
      nftMint: ctx.nftMint.publicKey,
      nftTokenAccount: ctx.nftTokenAccount,
      winnerAccount: ctx.winnerAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID

    }).signers([ctx.user1]).rpc();
    //await CheckCtx.getGameState(ctx);
  });

  it("Make move: LEFT!", async () => {
    await program.methods.makeMove({ left: {} }).accountsPartial({
      user: ctx.user1.publicKey,
      configAccount: ctx.configAccount,
      game: ctx.gameAccount,
      userAccount: ctx.user1Account,
      userTeamAccount: ctx.memeTeamAccount,
      userSeasonAccount: ctx.user1SeasonAccount,
      nftMint: ctx.nftMint.publicKey,
      nftTokenAccount: ctx.nftTokenAccount,
      winnerAccount: ctx.winnerAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID

    }).signers([ctx.user1]).rpc();
    await CheckCtx.getGameState(ctx);
  });

  it("Make move: LEFT!", async () => {
    await program.methods.makeMove({ left: {} }).accountsPartial({
      user: ctx.user1.publicKey,
      configAccount: ctx.configAccount,
      game: ctx.gameAccount,
      userAccount: ctx.user1Account,
      userTeamAccount: ctx.memeTeamAccount,
      userSeasonAccount: ctx.user1SeasonAccount,
      nftMint: ctx.nftMint.publicKey,
      nftTokenAccount: ctx.nftTokenAccount,
      winnerAccount: ctx.winnerAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID

    }).signers([ctx.user1]).rpc();

    await CheckCtx.getGameState(ctx);
  });

  it("Make move: UP!", async () => {
    await program.methods.makeMove({ up: {} }).accountsPartial({
      user: ctx.user1.publicKey,
      configAccount: ctx.configAccount,
      game: ctx.gameAccount,
      userAccount: ctx.user1Account,
      userTeamAccount: ctx.memeTeamAccount,
      userSeasonAccount: ctx.user1SeasonAccount,
      nftMint: ctx.nftMint.publicKey,
      nftTokenAccount: ctx.nftTokenAccount,
      winnerAccount: ctx.winnerAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID

    }).signers([ctx.user1]).rpc();

    await CheckCtx.getGameState(ctx);
  });

  it("Make move: UP!", async () => {
    await program.methods.makeMove({ up: {} }).accountsPartial({
      user: ctx.user1.publicKey,
      configAccount: ctx.configAccount,
      game: ctx.gameAccount,
      userAccount: ctx.user1Account,
      userTeamAccount: ctx.memeTeamAccount,
      userSeasonAccount: ctx.user1SeasonAccount,
      nftMint: ctx.nftMint.publicKey,
      nftTokenAccount: ctx.nftTokenAccount,
      winnerAccount: ctx.winnerAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID
    }).signers([ctx.user1]).rpc();

    await CheckCtx.getGameState(ctx);
  });

  it("Sumit to leaderboard", async () => {
    const tx = await program.methods.submitLeaderboard().accountsPartial({
      user: ctx.user1.publicKey,
      configAccount: ctx.configAccount,
      seasonAccount: ctx.seasonAccount,
      userAccount: ctx.user1Account,
      gameAccount: ctx.gameAccount,
      nftMint: ctx.nftMint.publicKey,
      systemProgram: SYSTEM_PROGRAM_ID
    }).signers([ctx.user1]).rpc()  ;

    const season = await CheckCtx.season(ctx);
    console.log("Season Leader board: ", season.leaderboard);
    await sleep(2000)
    const nftMedatadata = await getTokenMetadata(connection, ctx.nftMint.publicKey, 'confirmed', TOKEN_2022_PROGRAM_ID);
    console.log("🚀 ~ nftMedatadata: ", nftMedatadata);
  });

  it("Cannot create season if not ended", async () => {
    await sleep(3000);
    const configAccount = await CheckCtx.config(ctx);

    const currentSeasonId = configAccount.currentSeasonId;

    const seasonIdBuffer = Buffer.from([currentSeasonId + 1]);

    ctx.seasonAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("season"), seasonIdBuffer],
      program.programId
    )[0];

    ctx.memeTeamAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("meme_team"), seasonIdBuffer],
      program.programId
    )[0];

    ctx.chainTeamAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("chain_team"), seasonIdBuffer],
      program.programId
    )[0];
    const startTime = new anchor.BN(Date.now() / 1000);

    try {
      await program.methods.startSeason(startTime).accountsPartial({
        maintainer: ctx.maintainer.publicKey,
        configAccount: ctx.configAccount,
        seasonAccount: ctx.seasonAccount,
      }).signers([ctx.maintainer]).rpc();
    } catch (error) {
      assert.isTrue(error instanceof anchor.AnchorError);
      const err: anchor.AnchorError = error;
      expect(err.error.errorMessage).to.equal('SeasonNotEnded')
    }
  });
});
