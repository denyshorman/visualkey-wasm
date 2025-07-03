# VisualKey Rare Key Finder (WASM)

A WebAssembly (WASM) module for generating and searching for rare cryptographic keys, designed specifically for integration with the VisualKey app.  
Built with Rust, compiled to WASM, and ready for seamless use in JavaScript/TypeScript frontends.

## Features
- Generates random secp256k1 private keys
- Computes Ethereum addresses
- Calculates the rarity level (number of leading zero bits) of addresses
- Batch search for rare keys with a configurable threshold
- Finds an address matching a custom bitmask
- Exposes a simple WASM interface for use in a web worker or the main thread

## Usage

### 1. Build the WASM package

```
wasm-pack build --target web --out-dir pkg
```

### 2. Import and initialize in JavaScript/TypeScript

```js
import init, { generate_rare_keys_batch, find_address_with_mask } from 'find-rare-keys';

// Make sure to provide the correct path to the WASM file
await init({ module_or_path: '/assets/find_rare_keys_bg.wasm' });

const foundKeys = generate_rare_keys_batch(levelThreshold, batchSize);

// Example usage of find_address_with_mask:
const valueMask = new Uint8Array(20); // desired address bits
const careMask = new Uint8Array(20);  // which bits to care about (1 = must match, 0 = don't care)
const batchSize = 10000;              // number of attempts per call
const mask = new Uint8Array(44);
mask.set(valueMask, 0);
mask.set(careMask, 20);
mask.set(new Uint8Array([
  (batchSize >> 24) & 0xFF,
  (batchSize >> 16) & 0xFF,
  (batchSize >> 8) & 0xFF,
  batchSize & 0xFF
]), 40);

const result = find_address_with_mask(mask);
if (result) {
  // result.private_key and result.address are Uint8Arrays
}
```

### 3. Example: Using in a Web Worker

```js
import init, { generate_rare_keys_batch } from 'find-rare-keys';

onmessage = async ({ data }) => {
  await init({ module_or_path: '/assets/find_rare_keys_bg.wasm' });
  const foundKeys = generate_rare_keys_batch(data.levelThreshold, 1000);
  postMessage(foundKeys);
};
```

## API

### `generate_rare_keys_batch(levelThreshold: number, batchSize: number): Array<{ privateKey, address, level }>`
Finds keys with addresses that have at least `levelThreshold` leading zero bits.

### `find_address_with_mask(mask: Uint8Array): { privateKey, address } | null`
Finds a private key whose address matches a custom bitmask.

- `mask`: A 44-byte `Uint8Array`:
  - Bytes 0..20: `valueMask` (desired address bits)
  - Bytes 20..40: `careMask` (which bits to match: 1 = must match, 0 = don't care)
  - Bytes 40..44: `batchSize` (big-endian u32, number of attempts per call)
- Returns: An object `{ private_key, address }` if found, or `null` if not found in the batch.

> **Why a single mask parameter?**  
> Due to limitations and subtle pitfalls in how WebAssembly and `wasm-bindgen` handle multiple complex arguments (such as multiple `Uint8Array`s or a mix of arrays and numbers), passing all required data as a single packed buffer ensures robust and predictable interop between JavaScript and Rust.  
> When multiple arrays or mixed types are passed as separate parameters, memory allocation and glue code issues can cause arguments after the first to be received incorrectly (e.g., as zero-length arrays or zeros).  
> By combining all inputs into a single `Uint8Array`, we avoid these issues and guarantee correct data transfer.

> **Note:**  
> Ethereum addresses are always 160 bits (20 bytes), so both `valueMask` and `careMask` must be exactly 20 bytes each to fully specify which bits of the address to match or ignore.

#### Example

```js
const valueMask = new Uint8Array(20); // e.g., [0,0,0,0,...]
const careMask = new Uint8Array(20);  // e.g., [255,255,0,0,...] (only first two bytes must match)
const batchSize = 10000;
const mask = new Uint8Array(44);
mask.set(valueMask, 0);
mask.set(careMask, 20);
mask.set(new Uint8Array([
  (batchSize >> 24) & 0xFF,
  (batchSize >> 16) & 0xFF,
  (batchSize >> 8) & 0xFF,
  batchSize & 0xFF
]), 40);

const result = find_address_with_mask(mask);

if (result) {
  // result.private_key and result.address
}
```

## Installation in Frontend Projects

To use this package in your frontend project:

### 1. Install via npm
Install the package from the npm registry:

```
npm install @visualkey/find-rare-keys
```

### 2. Import and Use in Your App

```js
import init, { generate_rare_keys_batch } from '@visualkey/find-rare-keys';

await init({ module_or_path: '/path/to.wasm' });
const foundKeys = generate_rare_keys_batch(levelThreshold, batchSize);
```

## License

This project is licensed under the MIT License.
