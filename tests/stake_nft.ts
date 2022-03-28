import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StakeNft } from "../target/types/stake_nft";
import * as helpers from "./helpers";
describe("stake_nft", async() => {
  // Configure the client to use the local cluster.
  const connection = new anchor.web3.Connection(
    "http://localhost:8899",
    anchor.Provider.defaultOptions().preflightCommitment
  );

    const provider = helpers.getProvider(
    connection,
    anchor.web3.Keypair.generate()
  );
  const program = helpers.getProgram(provider);
  anchor.setProvider(anchor.Provider.env());

  // const program = anchor.workspace.StakeNft as Program<StakeNft>;
  const [blogAccount, blogAccountBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("blog_v0"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
