rust-starfish-wasm
======

A [\*><>](https://esolangs.org/wiki/Starfish) interpreter written in Rust. \*><> is a language derived from [><>](http://esolangs.org/wiki/Fish).

This is the WASM version, you can try it out [here](https://bafybeid6hawqv54mau4cdchhosb7zz36eotqcnsqwarqphv2sjihl5s3w4.ipfs.dweb.link/).

Building
---------------

Ensure the [Rust toolchain](https://www.rust-lang.org/tools/install) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) are installed. Then do the following:

```sh
git clone https://github.com/TheDiscordian/rust-starfish-wasm
cd rust-starfish-wasm
wasm-pack build --target web
```

That'll build the package, if you want to test it, run something like the following in the same directory to spin up an HTTP server:

```sh
python3 -m http.server
```

Finally, navigate to `http://localhost:8000/src/web` in your web browser to view the page.

Limitations
---------------

I believe file i/o is the only thing not working. Happy to take a PR for that. Any other issues are likely unexpected, please open an issue if you encounter one 🙂