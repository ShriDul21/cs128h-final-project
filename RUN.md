# Quantum Rust Setup

*The first step, as always:*
```
git clone (url)
```

*To simply view the compiled website, we need to get a static file server of your choice. 
One option is to go on VS Code Extensions and install Five Server. Then right-click the dist folder and select "Open with Five Server". A new webpage should open up in your browser with the app!*

*For developers wanting to test/modify the website locally, we need to use trunk. It will take a while since it builds from scratch*
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

*To host the website:* 
```bash
trunk serve --open 
```

