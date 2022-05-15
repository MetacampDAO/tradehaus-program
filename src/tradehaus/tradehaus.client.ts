import * as anchor from '@project-serum/anchor';
import { BN, Idl, Program, AnchorProvider } from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import {
  Account,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { Tradehaus } from '../../target/types/tradehaus';
import { AccountUtils, toBN, isKp } from '../tradehaus-common';

export class TradehausClient extends AccountUtils {
  // @ts-ignore
  wallet: anchor.Wallet;
  provider!: anchor.Provider;
  tradehausProgram!: anchor.Program<Tradehaus>;

  constructor(
    conn: Connection,
    // @ts-ignore
    wallet: anchor.Wallet,
    idl?: Idl,
    programId?: PublicKey
  ) {
    super(conn);
    this.wallet = wallet;
    this.setProvider();
    this.setTradehausProgram(idl, programId);
  }

  setProvider() {
    this.provider = new AnchorProvider(
      this.conn,
      this.wallet,
      AnchorProvider.defaultOptions()
    );
    anchor.setProvider(this.provider);
  }

  setTradehausProgram(idl?: Idl, programId?: PublicKey) {
    //instantiating program depends on the environment
    if (idl && programId) {
      //means running in prod
      this.tradehausProgram = new anchor.Program<Tradehaus>(
        idl as any,
        programId,
        this.provider
      );
    } else {
      //means running inside test suite
      // @ts-ignore
      this.tradehausProgram = anchor.workspace.Tradehaus as Program<Tradehaus>;
    }
  }

  // --------------------------------------- fetch deserialized accounts

  async fetchGameAcc(game: PublicKey) {
    return this.tradehausProgram.account.game.fetch(game);
  }

  // --------------------------------------- find PDA addresses

  // --------------------------------------- find all PDA addresses

  // --------------------------------------- breed ops ixs

  async createGame(
    gameConfig: Keypair,
    host: PublicKey | Keypair,
    hostRewardAccount: PublicKey,
    rewardMint: PublicKey,
    rewardEscrow: PublicKey,
    join: number,
    start: number,
    end: number,
    winners: number,
    maxPlayers: number,
    rewardAmount: number,
    rewardEscrowBump: number
  ) {
    const signers = [gameConfig];
    if (isKp(host)) signers.push(<Keypair>host)
    const txSig = await this.tradehausProgram.methods.createGame(
      toBN(join),
      toBN(start),
      toBN(end), 
      winners, 
      toBN(maxPlayers),
      toBN(rewardAmount),
      rewardEscrowBump
    ).accounts({
      gameConfig: gameConfig.publicKey,
      host: isKp(host)? (<Keypair>host).publicKey : host,
      hostRewardAccount,
      rewardMint,
      rewardEscrow,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY
    }).signers(signers)
    .rpc();

    return { txSig };
  }
}