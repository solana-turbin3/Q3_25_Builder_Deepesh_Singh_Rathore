#[cfg(test)]
mod tests {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
    };
    use std::str::FromStr;

    #[test]
    fn keygen() {
        //Create a new Keypair
        let kp = Keypair::new();

        println!(
            "You have generated a new Solana Wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save you wallet, copy and past the following in to the JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        //import our keypair
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");

        //establishing connection
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");

        //RPC URL
        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Generate a signature from the keypir
        let pubkey = keypair.pubkey();

        let message_bytes = b"I verify my Solana Keypair!";

        let sig = keypair.sign_message(message_bytes);
        // let sig_hashed = hash(sig.as_ref());

        // Verify the signature using the public key
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        let to_pubkey = Pubkey::from_str("dEEv13eRjRQodutata5L5ammEh54mPTo3e8B4wNvjWy").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // // Send the transaction and print the TX
        // let signature = rpc_client
        //     .send_and_confirm_transaction(&transaction)
        //     .expect("Failed to send transaction");
        // println!(
        //     "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        //     signature
        // );

        // Get current balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        //Build a mock transaction to calculate fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        //Esitmate the transaction fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        //Create the final transaction with balance minus fee
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        //send transaction and verify
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");
        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }


    

}
