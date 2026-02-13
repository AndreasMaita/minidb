# Building for Next.js

## Prerequisites

Install wasm-pack:
```bash
cargo install wasm-pack
```

## Build Steps

1. Navigate to the minidb-engine directory:
```bash
cd minidb-engine
```

2. Build the WASM package for web:
```bash
wasm-pack build --target web
```

This will create a `pkg/` directory with:
- `minidb_engine.js` - JavaScript bindings
- `minidb_engine_bg.wasm` - Compiled WebAssembly
- `minidb_engine.d.ts` - TypeScript type definitions
- `package.json` - NPM package metadata

3. In your Next.js project, import the package:

```typescript
import init, { BPlusTreeWrapper } from './path/to/pkg/minidb_engine';

// In a component or page
useEffect(() => {
  init().then(() => {
    const tree = new BPlusTreeWrapper(4);
    // Use the tree...
  });
}, []);
```

## Alternative: Publish to NPM

To use it as a proper NPM package:

1. Build with target bundler:
```bash
wasm-pack build --target bundler
```

2. Publish to NPM:
```bash
cd pkg
npm publish
```

3. Install in Next.js:
```bash
npm install minidb-engine
```

## Notes

- The WASM module must be initialized with `init()` before use
- In Next.js, do this in `useEffect` or a client component
- TypeScript types are automatically generated
