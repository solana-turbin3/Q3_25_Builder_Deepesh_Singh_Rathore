import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { BN } from "bn.js";

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  let wallet = provider.wallet;


  const program = anchor.workspace.vault as Program<Vault>;

  it("Vault initialized!", async () => {
    // Add your test here.

   const [vaultStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),wallet.publicKey.toBuffer()],
      program.programId
    )

    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),vaultStatePDA.toBuffer()],
      program.programId
    )

    try{

      const tx = await program.methods.initialize()
      .accounts({
        user : wallet.publicKey
      })
      .rpc();
      
      
      console.log("Your transaction signature", tx);
      const vaultStateAccount = await program.account.vaultState.fetch(vaultStatePDA);
      console.log(`Vault State Bump : ${vaultStateAccount.stateBump} Vault Bump : ${vaultStateAccount.vaultBump}`)
      
      const vaultAccount = await program.provider.connection.getAccountInfo(vaultPDA);
      console.log("Vautl Details ",vaultAccount);
    
    }catch(e){
     console.error(e);
    }

  });

  it("Deposit to vault", async()=>{
    const [vaultStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),wallet.publicKey.toBuffer()],
      program.programId
    )

    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),vaultStatePDA.toBuffer()],
      program.programId
    )
    const amount = anchor.web3.LAMPORTS_PER_SOL;
    await program.methods.deposit(new BN(amount)).accounts({
      user : wallet.publicKey
    }).rpc();

    const userVault = await program.provider.connection.getAccountInfo(vaultPDA);
    console.log(userVault);
  })
});
