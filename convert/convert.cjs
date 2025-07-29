"use strict";
exports.__esModule = true;
// convertFromBase58.ts
var bs58_1 = require("bs58");
/**
 * Converts a base58 Solana private key string to Uint8Array
 * @param base58Key - The base58-encoded private key string from Phantom
 * @returns Uint8Array of private key
 */
function base58ToByteArray(base58Key) {
    return bs58_1["default"].decode(base58Key);
}
// Example usage:
var base58PrivateKey = '3TKTYseHGWcmMvqu1sF7ZpVonRPPMgijqUT1WLUePktqkpstEpNpdLhpeHcqFAeU25KQs28KJoa1FdTkEx4aaYAD'; // Your Phantom-exported base58 string
var privateKeyBytes = base58ToByteArray(base58PrivateKey);
console.log('Private Key Bytes:', privateKeyBytes);
