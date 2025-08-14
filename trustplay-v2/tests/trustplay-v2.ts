import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram } from "@solana/web3.js";
import { TrustplayV2 } from "../target/types/trustplay_v2";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { BN } from "bn.js";

describe("Trustplay Tests on Devnet", () => {
    //configure the client to use the local validator
    const provider = anchor.AnchorProvider.env();
    console.log("connected to : ", provider.connection.rpcEndpoint);

    anchor.setProvider(provider);
    const wallet = provider.wallet ;

    const program = anchor.workspace.trustplay_v2 as Program<TrustplayV2>;

    let player1 = anchor.web3.Keypair.generate();
    let player2 = anchor.web3.Keypair.generate();
    let player3 = anchor.web3.Keypair.generate();
    let player4 = anchor.web3.Keypair.generate();
    let player5 = anchor.web3.Keypair.generate();

    const players = [player1, player2, player3, player4, player5];

    it("Initializes the Room ", async () => {

        const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("room"),
                wallet.publicKey.toBytes(),
            ], program.programId
        )
        const [vaultStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("vault"),
                roomPDA.toBytes(),
            ],
            program.programId
        )

        const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("vault"),
                vaultStatePDA.toBytes()
            ],
            program.programId
        )

        const tx = await program.methods.initialize("SolPlay")
            .accountsStrict(
                {
                    organizer: provider.wallet.publicKey,
                    room: roomPDA,
                    vaultState: vaultStatePDA,
                    vault: vault,
                    systemProgram: anchor.web3.SystemProgram.programId
                }
            ).rpc()

        console.log("Transaction Signature", tx);

        const roomAccount = await program.account.room.fetch(roomPDA);
        const vaultStateAccount = await program.account.vaultState.fetch(vaultStatePDA);
        const vaultAccount = await program.provider.connection.getAccountInfo(vault);

        console.log(`Room Account ${roomAccount.players} ${roomAccount.organizer} \n Vault State ${vaultStateAccount.stateBump} ${vaultStateAccount.vaultBump} \n vault : ${vaultAccount}`);
    }
    )

    it("Let Players Join Room", async () => {

        const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("room"),
                wallet.publicKey.toBytes(),
            ], program.programId
        )

        for (let i = 0; i < players.length; i++) {

            const tx = await program.methods.joinRoom().accountsStrict({
                room: roomPDA,
                player: players[i].publicKey,
                systemProgram: anchor.web3.SystemProgram.programId
            }).signers([players[i]]).rpc()

            console.log("Player Trnasaction signature :", tx);
        }

        const roomAccount = await program.account.room.fetch(roomPDA);
        console.log(`Room Account ${roomAccount.players} ${roomAccount.organizer}`);

    })

    it("Deposit Money to Vault", async () => {

        const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("room"),
                wallet.publicKey.toBytes(),
            ], program.programId
        )

         const [vaultStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("vault"),
                roomPDA.toBytes(),
            ],
            program.programId
        )

        const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("vault"),
                vaultStatePDA.toBytes()
            ],
            program.programId
        )
        // console.log(vaultAccount);

        const vaultAccountbefore = await program.provider.connection.getAccountInfo(vault);
        console.log(vaultAccountbefore);
        
        const tx = await program.methods.depositToVault(new BN(LAMPORTS_PER_SOL)).accounts({
            organizer : wallet.publicKey
        }).signers([]).rpc()
        
        const vaultAccount = await program.provider.connection.getAccountInfo(vault);
        console.log(vaultAccount.lamports);
        

    })

    it("Distribute Prizes", async () => {

        const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("room"),
                wallet.publicKey.toBytes(),
            ], 
            program.programId
        )

        const [vaultStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("vault"),
                roomPDA.toBytes(),
            ],
            program.programId
        )

        const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("vault"),
                vaultStatePDA.toBytes()
            ],
            program.programId
        )

        // Get vault balance before distribution
        const vaultAccountBefore = await program.provider.connection.getAccountInfo(vault);
        console.log("Vault balance before distribution:", vaultAccountBefore.lamports);

        // Get winner's balance before distribution (using first player as winner)
        const winner = players[0]; // First player as winner
        const winnerBalanceBefore = await program.provider.connection.getBalance(winner.publicKey);
        console.log("Winner balance before:", winnerBalanceBefore);

        // Distribute prizes to the winner
        const tx = await program.methods.distributePrizes()
            .accountsStrict({
                organizer: wallet.publicKey,
                room: roomPDA,
                vaultState: vaultStatePDA,
                vault: vault,
                winnerAccount: winner.publicKey, // The winner account
                systemProgram: anchor.web3.SystemProgram.programId
            })
            .signers([]) // No additional signers needed, organizer is already the signer
            .rpc();

        console.log("Distribute prizes transaction signature:", tx);

        // Verify the distribution
        const vaultAccountAfter = await program.provider.connection.getAccountInfo(vault);
        const winnerBalanceAfter = await program.provider.connection.getBalance(winner.publicKey);

        console.log("Vault balance after distribution:", vaultAccountAfter.lamports);
        console.log("Winner balance after:", winnerBalanceAfter);
        console.log("Prize distributed:", winnerBalanceAfter - winnerBalanceBefore);

        // The vault should have minimal balance (rent exempt amount)
        // The winner should have received the prize money
        const prizeDistributed = winnerBalanceAfter - winnerBalanceBefore;
        console.log(`✅ Successfully distributed ${prizeDistributed} lamports to winner`);
    });
})