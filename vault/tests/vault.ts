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
    const initialBalance = await program.provider.connection.getBalance(vaultPDA);
    
    try {
      await program.methods.deposit(new BN(amount)).accounts({
        user : wallet.publicKey
      }).rpc();

      const finalBalance = await program.provider.connection.getBalance(vaultPDA);
      console.log(`Initial balance: ${initialBalance}, Final balance: ${finalBalance}`);
      console.log(`Deposited: ${finalBalance - initialBalance} lamports`);
      
    } catch(e) {
      console.error("Deposit error:", e);
    }
  })

  it("Withdraw from vault", async()=>{
    const [vaultStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),wallet.publicKey.toBuffer()],
      program.programId
    )

    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),vaultStatePDA.toBuffer()],
      program.programId
    )
    
    const withdrawAmount = anchor.web3.LAMPORTS_PER_SOL / 2; // Withdraw half a SOL
    const initialVaultBalance = await program.provider.connection.getBalance(vaultPDA);
    const initialUserBalance = await program.provider.connection.getBalance(wallet.publicKey);
    
    try {
      await program.methods.withdraw(new BN(withdrawAmount)).accounts({
        user : wallet.publicKey
      }).rpc();

      const finalVaultBalance = await program.provider.connection.getBalance(vaultPDA);
      const finalUserBalance = await program.provider.connection.getBalance(wallet.publicKey);
      
      console.log(`Vault balance before: ${initialVaultBalance}, after: ${finalVaultBalance}`);
      console.log(`User balance before: ${initialUserBalance}, after: ${finalUserBalance}`);
      console.log(`Withdrawn: ${initialVaultBalance - finalVaultBalance} lamports`);
      
    } catch(e) {
      console.error("Withdraw error:", e);
    }
  })

  it("Close vault account", async()=>{
    const [vaultStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),wallet.publicKey.toBuffer()],
      program.programId
    )

    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),vaultStatePDA.toBuffer()],
      program.programId
    )
    
    const initialVaultBalance = await program.provider.connection.getBalance(vaultPDA);
    const initialUserBalance = await program.provider.connection.getBalance(wallet.publicKey);
    
    try {
      await program.methods.close().accounts({
        signer: wallet.publicKey
      }).rpc();

      const finalUserBalance = await program.provider.connection.getBalance(wallet.publicKey);
      
      // Check that vault account is closed
      const vaultAccount = await program.provider.connection.getAccountInfo(vaultPDA);
      const vaultStateAccount = await program.provider.connection.getAccountInfo(vaultStatePDA);
      
      console.log(`Initial vault balance: ${initialVaultBalance}`);
      console.log(`User balance before: ${initialUserBalance}, after: ${finalUserBalance}`);
      console.log(`Vault account after close: ${vaultAccount}`);
      console.log(`Vault state account after close: ${vaultStateAccount}`);
      
      // Verify accounts are closed
      if (vaultAccount === null) {
        console.log("✅ Vault account successfully closed");
      }
      if (vaultStateAccount === null) {
        console.log("✅ Vault state account successfully closed");
      }
      
    } catch(e) {
      console.error("Close account error:", e);
    }
  })
});
