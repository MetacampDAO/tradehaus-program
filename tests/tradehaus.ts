import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tradehaus } from "../target/types/tradehaus";
import { PublicKey, SystemProgram, Transaction, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, mintTo, getAccount} from "@solana/spl-token";
import chai, { assert, expect } from 'chai';
import chaiAsPromised from 'chai-as-promised';

import { TradehausClient } from '../src';

chai.use(chaiAsPromised);

describe("tradehaus", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const th = new TradehausClient(provider.connection, provider.wallet as any)

  let mintReward = null;

  let hostTokenAccountReward = null;
  let player1TokenAccountReward = null;
  let player2TokenAccountReward = null;
  let player3TokenAccountReward = null;
  
  let reward_escrow_pda: PublicKey = null; // escrow account stores reward tokens
  let reward_escrow_bump: number = null;
  let escrow_authority_pda = null;

  const host = anchor.web3.Keypair.generate();
  const player1 = anchor.web3.Keypair.generate();
  const player2 = anchor.web3.Keypair.generate();
  const player3 = anchor.web3.Keypair.generate();

  // generate keypair for game config account
  const gameConfig = anchor.web3.Keypair.generate();

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
    mintReward = await createMint(
      provider.connection,
      host,
      host.publicKey,
      null,
      0
    );

    // create host & players reward token accounts
    hostTokenAccountReward = await createAssociatedTokenAccount(
      provider.connection,
      host,
      mintReward,
      host.publicKey
    );

    player1TokenAccountReward = await createAssociatedTokenAccount(
      provider.connection,
      player1,
      mintReward,
      player1.publicKey
    );  
    
    player2TokenAccountReward = await createAssociatedTokenAccount(
      provider.connection,
      player2,
      mintReward,
      player2.publicKey
    ); 
    
    player3TokenAccountReward = await createAssociatedTokenAccount(
      provider.connection,
      player3,
      mintReward,
      player3.publicKey
    ); 

    // mint to host token account
    await mintTo(
      provider.connection,
      host,
      mintReward,
      hostTokenAccountReward,
      host,
      30
    );

    let _hostTokenAccountReward = await getAccount(
      provider.connection,
      hostTokenAccountReward
    )

    // assert ok to check amount of tokens in host is correct
    assert.ok(Number(_hostTokenAccountReward.amount) == 30);
  });

  it ('creates game', async () => {
    const [reward_escrow_pda, reward_escrow_bump] = await th.findRewardEscrowPDA(gameConfig.publicKey);

    const _START_TIME = Math.ceil(Date.now()/1000 + 30) 

    await th.createGame(
      gameConfig,
      host,
      hostTokenAccountReward,
      mintReward,
      reward_escrow_pda,
      1,
      _START_TIME,
      2000000000,
      100000,
      3,
      3,
      30,
      reward_escrow_bump
    );

    const gameAcc = await th.fetchGameAcc(gameConfig.publicKey);
    let _rewardEscrow = await getAccount(
      provider.connection,
      reward_escrow_pda
    );

    let _hostTokenAccountReward = await getAccount(
      provider.connection,
      hostTokenAccountReward
    );

    assert.equal(gameAcc.host.toBase58(), host.publicKey.toBase58())
    assert.equal(gameAcc.hostRewardAccount.toBase58(), hostTokenAccountReward.toBase58())
    assert.equal(gameAcc.rewardMint.toBase58(), mintReward.toBase58())
    assert.equal(gameAcc.rewardEscrow.toBase58(), reward_escrow_pda.toBase58())

    assert.ok(gameAcc.rewardAmount.toNumber() == 30)
    assert.ok(gameAcc.joinTime.toNumber() == 1)
    assert.ok(gameAcc.startTime.toNumber() == _START_TIME)
    assert.ok(gameAcc.endTime.toNumber() == 2000000000)
    assert.ok(gameAcc.startUsd.toNumber() == 100000)
    assert.ok(gameAcc.currentCap.toNumber() == 0)
    assert.ok(gameAcc.maxCap.toNumber() == 3)
    assert.ok(gameAcc.winners == 3)
    assert.ok(gameAcc.rewardEscrowBump == reward_escrow_bump)

    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("authority-seed")),
        gameConfig.publicKey.toBytes()
      ],
      th.tradehausProgram.programId
    );

    assert.ok(Number(_hostTokenAccountReward.amount) == 0);
    assert.ok(_rewardEscrow.owner.equals(_vault_authority_pda));
    assert.ok(Number(_rewardEscrow.amount) == 30);  


  });

  it('join game', async () => {
    const [player_fund_pda, player_fund_bump] = await th.findPlayerFundPDA(
      player1.publicKey,
      gameConfig.publicKey
    );

    await th.joinGame(
      gameConfig.publicKey,
      player1,
      player_fund_pda,
      player_fund_bump
    );

    const _player1FundAcc = await th.fetchFundAcc(player_fund_pda);
    const _gameAcc = await th.fetchGameAcc(gameConfig.publicKey);

    // test that all coins initiate to 0
    assert.ok(Number(_player1FundAcc.btcQty) == 0);
    assert.ok(Number(_player1FundAcc.ethQty) == 0);
    assert.ok(Number(_player1FundAcc.linkQty) == 0);
    assert.ok(Number(_player1FundAcc.solQty) == 0);

    // test that player starting USD matches game configs'
    assert.ok(Number(_player1FundAcc.usdQty) == 100000);

    // test that player fund bump is being stored
    assert.ok(_player1FundAcc.fundBump == player_fund_bump);

    // test that current cap is being incremented
    assert.ok(Number(_gameAcc.currentCap) == 1);
  })

  it('swap items', async () => {
    const [player_fund_pda, player_fund_bump] = await th.findPlayerFundPDA(
      player1.publicKey,
      gameConfig.publicKey,
    );

    await th.swapItems(
      playerFund,
      player1.publicKey,
      gameConfig.publicKey,
      100,
      1,
      2,
    );

    const _player1FundAcc = await th.fetchFundAcc(player_fund_pda);
    const _gameAcc = await th.fetchFundAcc(gameConfig.publicKey);

    //test 
    assert.ok(Number);
  })


});