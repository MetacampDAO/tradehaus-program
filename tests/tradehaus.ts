import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tradehaus } from "../target/types/tradehaus";
import { PublicKey, SystemProgram, Transaction, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMintToCheckedInstruction } from "@solana/spl-token";
import chai, { assert, expect } from 'chai';
import chaiAsPromised from 'chai-as-promised';

chai.use(chaiAsPromised);

describe("tradehaus", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Tradehaus as Program<Tradehaus>;

  let mintReward = null;

  let hostTokenAccountReward = null;
  let player1TokenAccountReward = null;
  let player2TokenAccountReward = null;
  let player3TokenAccountReward = null;
  
  let escrow_account_pda = null; // escrow account stores reward tokens
  let escrow_account_bump = null;
  let escrow_authority_pda = null;

  const host = anchor.web3.Keypair.generate();
  const player1 = anchor.web3.Keypair.generate();
  const player2 = anchor.web3.Keypair.generate();
  const player3 = anchor.web3.Keypair.generate();



  it("Initialize mint and token accounts", async () => {
    // airdrop to host
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(host.publicKey, 1000000000),
      "confirmed"
    );

    // fund players account
    await provider.sendAndConfirm(
      (() => {
        const tx = new Transaction();
        tx.add(
          SystemProgram.transfer({
            fromPubkey: host.publicKey,
            toPubkey: player1.publicKey,
            lamports: 100000000,
          }),
          SystemProgram.transfer({
            fromPubkey: host.publicKey,
            toPubkey: player2.publicKey,
            lamports: 100000000,
          }),
          SystemProgram.transfer({
            fromPubkey: host.publicKey,
            toPubkey: player3.publicKey,
            lamports: 100000000,
          }),
        );
        return tx;
      })(),
      [host]
    );

    // create mint of reward token

    // create host & players reward token accounts

    // mint to host token account

    // assert ok to check amount of tokens in host is correct\

  });


});