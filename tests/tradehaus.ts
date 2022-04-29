import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tradehaus } from "../target/types/tradehaus";

describe("tradehaus", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tradehaus as Program<Tradehaus>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
