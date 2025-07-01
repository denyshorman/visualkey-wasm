# VisualKey Rare Key Finder (WASM)

A WebAssembly (WASM) module for generating and searching for rare cryptographic keys, designed specifically for integration with the VisualKey app.  
Built with Rust, compiled to WASM, and ready for seamless use in JavaScript/TypeScript frontends.

## Features
- Generates random secp256k1 private keys
- Computes Ethereum-style addresses
- Calculates the rarity level (number of leading zero bits) of addresses
- Batch search for rare keys with a configurable threshold
- Exposes a simple WASM interface for use in web workers or main thread

## Usage

### 1. Build the WASM package

```
wasm-pack build --target web --out-dir pkg
```

### 2. Import and initialize in JavaScript/TypeScript

```js
import init, { find_rare_key_batch } from 'find-rare-key';

// Make sure to provide the correct path to the WASM file
await init('/assets/find_rare_key_bg.wasm');

const result = find_rare_key_batch(levelThreshold, batchSize);
```

### 3. Example: Using in a Web Worker

```js
import init, { find_rare_key_batch } from 'find-rare-key';

onmessage = async ({ data }) => {
  await init('/assets/find_rare_key_bg.wasm');
  const result = find_rare_key_batch(data.levelThreshold, 1000);
  postMessage({ foundKeys: result });
};
```

## Installation in Frontend Projects

To use this package in your frontend project:

### 1. Install via npm
Install the package from the npm registry:

```
npm install @visualkey/find-rare-key
```

### 2. Import and Use in Your App

```js
import init, { find_rare_key_batch } from '@visualkey/find-rare-key';

await init('/path/to.wasm');
const result = find_rare_key_batch(levelThreshold, batchSize);
```

## License

This project is licensed under the MIT License.
