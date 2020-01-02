# ARWA: A Rusty Web-API

This crate is an experiment with wrapping the [web_sys](https://crates.io/crates/web-sys)
APIs with an API that better aligns with Rust idioms and conventions (as established in `core`
/`std`). In its current state, this implementation should be considered merely an exploration of
the possibilities and pitfalls: it is barely tested, undocumented, and expected to change
significantly. Any contributions, ideas, feedback and discussion would be greatly appreciated,
please feel free to open new [issues](https://github.com/RSSchermer/ARWA/issues).

## Getting started

The [examples](examples) directory contains several small `wasm-bindgen` example applications. 
For now the best way to get started is to copy an example. To install the NodeJS dependencies 
run:

```bash
npm install
```

Then run the application with:

```bash
npm run serve
```

You should now be able to find the result in a browser at [localhost:8080](http://localhost:8080).

Note that this requires [NodeJS](https://nodejs.org/en/).

## License

MIT
