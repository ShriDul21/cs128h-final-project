# Quantum Rust Setup

*To simply view the compiled website, you can visit the website below or use a static file server on the /dist folder after cloning*
```
https://quantumrust.vercel.app/
```

*For developers, we proceed normally:*
```
git clone (url)
```
*We need to use trunk to render the website. It will take a while since it builds from scratch*
```bash
cargo install trunk 
```

*Run this so the code will compile to webassembly and actually render a website*
```bash
rustup target add wasm32-unknown-unknown
```

*If you get any errors about mismatched ABI, run this. (Unsure if this is a fatal error, but better safe than sorry)* 
```bash
rustup override set 1.89.0
```

*To host the website locally:* 
```bash
trunk serve --open 
```

