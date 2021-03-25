Build wasm:
```../wasm-test> wasm-pack build```

Link pkg to make it available:
```../wasm-test/pkg> npm link```

Link site to 'wasm-test':
```../wasm-test/site> npm link wasm-test```

Install dependencies:
```../wasm-test/site> npm install```

Run:
```../wasm-test/site> npm run start```