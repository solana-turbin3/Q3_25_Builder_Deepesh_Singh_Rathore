import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Joinroom } from "../target/types/joinroom";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("joinroom", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  // Wallet provided by localnet
  const wallet = anchor.AnchorProvider.env().wallet as NodeWallet;

  const program = anchor.workspace.joinroom as Program<Joinroom>;

  it("Initializes a room", async () => {

    const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("room"), wallet.publicKey.toBytes()],
      program.programId,
    )

    const tx = await program.methods.initialize("Sol_Play")
      .accountsStrict(
        {
          room: roomPDA,
          creator: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        }
      ).rpc()

    console.log("Transaction Signature :", tx);

    const roomAccount = await program.account.room.fetch(roomPDA);
    console.log(`Room Data : ${roomAccount} Room Address: ${roomPDA}`);
  });

  it("Player joins room.", async () => {

    const newPlayer = anchor.web3.Keypair.generate();
    const signature = await anchor.AnchorProvider.env().connection.requestAirdrop(newPlayer.publicKey, anchor.web3.LAMPORTS_PER_SOL);

    await anchor.AnchorProvider.env().connection.confirmTransaction(signature, "confirmed");


    const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("room"), wallet.publicKey.toBytes()],
      program.programId,
    )

    const [participantPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("participant"), roomPDA.toBuffer(), newPlayer.publicKey.toBuffer()],
      program.programId
    )

    const tx = await program.methods.joinroom()
      .accountsStrict({
        participantpda: participantPDA,
        room: roomPDA,
        participant: newPlayer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([newPlayer])
      .rpc()

    try {
      const tx = await program.methods.joinroom()
        .accountsStrict({
          participantpda: participantPDA,
          room: roomPDA,
          participant: newPlayer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([newPlayer])
        .rpc()
      console.error("❌ Should not allow duplicate join!");
    } catch (err) {
      console.log("✅ Duplicate join prevented:", err.message);
    }
    const ParticipantAccount = await program.account.participant.fetch(participantPDA);
    console.log("Player Details \n ", ParticipantAccount)
  })

  it("Prevents more than 8 Players from joining the same room", async () => {

    //create a player array 
    const players: anchor.web3.Keypair[] = [];

    for (let i = 0; i < 8; i++) {


      const playerKeyPair = anchor.web3.Keypair.generate();
      const signature = await anchor.AnchorProvider.env().connection.requestAirdrop(playerKeyPair.publicKey, anchor.web3.LAMPORTS_PER_SOL);

      await anchor.AnchorProvider.env().connection.confirmTransaction(signature, "confirmed");

      players.push(playerKeyPair)

    }

    for (const player of players) {

      const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("room"), wallet.publicKey.toBytes()],
        program.programId,
      )

      const [participantPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("participant"), roomPDA.toBuffer(), player.publicKey.toBuffer()],
        program.programId
      )

      await program.methods.joinroom().accountsStrict(
        {
          participantpda: participantPDA,
          room: roomPDA,
          participant: player.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        }
      ).signers([player]).rpc()

      console.log(`✅ Player ${player.publicKey.toBase58()} joined.`);
    }

    try {
      let ninethPlayer = anchor.web3.Keypair.generate();
      let signature = await anchor.AnchorProvider.env().connection.requestAirdrop(ninethPlayer.publicKey, anchor.web3.LAMPORTS_PER_SOL);
      await anchor.AnchorProvider.env().connection.confirmTransaction(signature, "confirmed");

      const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("room"), wallet.publicKey.toBytes()],
        program.programId,
      )

      const [participantPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("participant"), roomPDA.toBuffer(), ninethPlayer.publicKey.toBuffer()],
        program.programId
      )

      await program.methods.joinroom().accountsStrict(
        {
          participantpda: participantPDA,
          room: roomPDA,
          participant: ninethPlayer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        }
      ).signers([ninethPlayer]).rpc()
      console.error("❌ Should not allow 9th player to join!");
    } catch (err) {
      console.log("✅ 9th player blocked as expected:", err.message);
    }
  })

  it("Initializes the vault with a price pool", async () => {

    const amount = anchor.web3.LAMPORTS_PER_SOL;

    const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("room"), wallet.publicKey.toBuffer()],
      program.programId
    )
    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), wallet.publicKey.toBuffer(), roomPDA.toBuffer()],
      program.programId
    )

    await program.methods.initializeVault(new anchor.BN(amount)).accountsStrict(
      {
        room: roomPDA,
        vault: vaultPDA,
        creator: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    ).rpc()

    const vaultAccount = await program.account.vault.fetch(vaultPDA);
    console.log("Vault Details:", {
      balance: vaultAccount.balance.toString(),
      isLocked: vaultAccount.isLocked,
      authority: vaultAccount.vaultAuthority.toBase58(),
      room: vaultAccount.room.toBase58(),
    });

    const roomAccount = await program.account.room.fetch(roomPDA);
    console.log("Room Details:", {
      name: roomAccount.name,
      creator: roomAccount.creator.toBase58(),
    });
  })

  it("Distribute the prices", async()=>{

    const [roomPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("room"), wallet.publicKey.toBuffer()],
      program.programId
    )
    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), wallet.publicKey.toBuffer(), roomPDA.toBuffer()],
      program.programId
    )



    await program.methods.distributePrize().accountsStrict(
      {
        room : roomPDA,
        vault : vaultPDA,
        creator : wallet.publicKey

      }
    ).accounts().rpc()
  })
});
