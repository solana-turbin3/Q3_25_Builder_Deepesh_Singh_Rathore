// convertFromBase58.ts
import bs58 from 'bs58';

/**
 * Converts a base58 Solana private key string to Uint8Array
 * @param base58Key - The base58-encoded private key string from Phantom
 * @returns Uint8Array of private key
 */
function base58ToByteArray(base58Key: string): Uint8Array {
  return bs58.decode(base58Key);
}

// Example usage:
const base58PrivateKey = '3TKTYseHGWcmMvqu1sF7ZpVonRPPMgijqUT1WLUePktqkpstEpNpdLhpeHcqFAeU25KQs28KJoa1FdTkEx4aaYAD'; // Your Phantom-exported base58 string
const privateKeyBytes = base58ToByteArray(base58PrivateKey);

console.log('Private Key Bytes:', privateKeyBytes);
