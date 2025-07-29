import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, IdlError } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";

import wallet from './Turbin3-wallet.json'
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");


const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a solana devnet connection 
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed" });

const program: Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const account_seeds = [
    Buffer.from('prereqs'),
    keypair.publicKey.toBuffer(),
];
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

const authority_seeds = [
    Buffer.from('collection'),
    mintCollection.toBuffer(),
];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);

const [authority,_authority_bump] = PublicKey.findProgramAddressSync(authority_seeds,program.programId);

const mintTs = Keypair.generate();


// Execute the initialize transaction

// (async () => {
//     try {
//         const txhash = await program.methods
//             .initialize("deepesh-sr")
//             .accountsPartial({
//                 user: keypair.publicKey,
//                 account: account_key,
//                 system_program: "11111111111111111111111111111111",
//             })
//             .signers([keypair])
//             .rpc();
//         console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
//     } catch (e) {
//         console.error(`Oops, something went wrong: ${e}`);
//     }
// })();

// Execut the submitTs Transction 

(async ()=>{
    try{
        const txhash = await program.methods
            .submitRs()
            .accountsPartial({
            user :keypair.publicKey,
            account:account_key,
            mint:mintTs.publicKey,
            collection:mintCollection,
            authority:authority,
            mpl_core_program:MPL_CORE_PROGRAM_ID,
            system_program:"11111111111111111111111111111111"
        })
        .signers([keypair,mintTs])
        .rpc();

        console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }catch(e){
        console.error(`Oops, something went wrong ${e}`)
    }
})();
