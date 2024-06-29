import { Connection, Keypair, PublicKey, Signer } from "@solana/web3.js"
import { FomoloveSonic } from "../../target/types/fomolove_sonic"
import { Program } from "@coral-xyz/anchor"
import { createUserWithLamports } from "./helper";
import bs58 from 'bs58';

const hexString = '0000000000000000000000000000000000000000000000000000000000000000'; // 32 bytes in hex
const buffer = Buffer.from(hexString, 'hex');

export interface Ctx {
  connection: Connection,
  program: Program<FomoloveSonic>,
  maintainer: Signer,
  user1: Signer,
  user2: Signer,
  configAccount: PublicKey,
  winnerAccount: PublicKey,
  seasonAccount: PublicKey,
  memeTeamAccount: PublicKey,
  chainTeamAccount: PublicKey,
  user1Account: PublicKey,
  user2Account: PublicKey,
  user1SeasonAccount: PublicKey,
  gameAccount: PublicKey,
  nftMint: Keypair,
  nftTokenAccount: PublicKey
}

export async function createCtx(connection: Connection, program: Program<FomoloveSonic>): Promise<Ctx> {

  const maintainer = await createUserWithLamports(connection, 10);
  const user1 = await createUserWithLamports(connection, 10);
  const user2 = await createUserWithLamports(connection, 10);
  const nftMint = Keypair.generate();

  const configAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  )[0];

  const winnerAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("winner")],
    program.programId
  )[0];

  const seasonAccount = new PublicKey(bs58.encode(buffer));

  const memeTeamAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("meme_team")],
    program.programId
  )[0];

  const chainTeamAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("chain_team")],
    program.programId
  )[0];

  let nftTokenAccount = new PublicKey(bs58.encode(buffer));
  const gameAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("game"), nftMint.publicKey.toBuffer()],
    program.programId
  )[0];
  
  const user1Account =  PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user1.publicKey.toBuffer()],
    program.programId
  )[0];

  const user2Account =  PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user2.publicKey.toBuffer()],
    program.programId
  )[0];
  const user1SeasonAccount = new PublicKey(bs58.encode(buffer));

  return {
    connection, 
    program,
    maintainer,
    user1,
    user2,
    configAccount,
    winnerAccount,
    seasonAccount,
    memeTeamAccount,
    chainTeamAccount,
    user1Account,
    user2Account,
    user1SeasonAccount,
    gameAccount,
    nftMint,
    nftTokenAccount
  }

}