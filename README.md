# Arwa: A(nother) Rusty Web-API

The Arwa project aims to expose modern web-APIs as Rust APIs, allowing you to interact with browser functionality such 
as the DOM, the browser console, the fetch API, and many more. Arwa builds on top of 
[web-sys](https://crates.io/crates/web-sys). `web-sys` by itself also exposes the browser's web-APIs; it achieves this
by automatically generating rust bindings from the [WebIDL](https://webidl.spec.whatwg.org/) specifications of the 
web-APIs. However, APIs designed for Javascript do not always map neatly to Rust idioms. Arwa is an attempt at providing 
a better developer experience, by tailoring more opinionated decisions on how to best expose web-functionality in Rust 
on a case by case basis. A core guiding concept in the design of this mapping is the following "hierarchy of 
consistency":

1. Arwa first and foremost tries to be internally consistent (even when the web-standards themselves not always quite
   manage to be).
2. Secondly, Arwa tries to be consistent with the Rust ecosystem, in particular with [std](https://doc.rust-lang.org/std/),
   the Rust standard library. As a simple example, Arwa uses `len` - not `length` - for the size of a collection.
3. Lastly, Arwa tries to be consistent with the Javascript web-APIs.

This hierarchy means that Arwa may not always closely follow the idioms of the Javascript APIs, though hopefully those 
versed in interacting with the browser through Javascript will hopefully quickly find it familiar.

As Arwa builds on top of `web-sys`, most Arwa types can be converted to a corresponding `web-sys` type through a 
[From](https://doc.rust-lang.org/std/convert/trait.From.html) / [Into](https://doc.rust-lang.org/std/convert/trait.Into.html) 
conversion (which is almost always zero-cost). This means that if Arwa does not (yet) implement a certain web API, but 
`web_sys` does, you will still be able to interact with this API by "lowering" into `web_sys` types and using "raw"
`web_sys`. It also means that you can use other crates that build on top of `web_sys` and interoperate with Arwa.

## Project Status

Arwa is still very young and thus the API is in still in flux. Arwa currently has (mostly) complete APIs for the 
following set of web APIs (as distinguished by [MDN](https://developer.mozilla.org/en-US/docs/Web/API)):

- CSSOM
- Console API
- DOM
- Fetch API
- File API
- Fullscreen API
- Geolocation API
- HTML DOM (including HTML Media Elements and Custom Elements)
- HTML Drag And Drop API
- History API
- Page Visibility API
- Performance API
- Pointer Events
- Pointer Lock API
- Resource Timing API
- Selection API
- Service Worker API (without `postMessage`, see below)
- UI Events
- URL API (currently only parsed from strings, no builder API yet)
- Vibration API
- Web Crypto API (but not "subtle" crypto; random number generation only)
- Web Storage API
- Web Workers API (without `postMessage`, see below)

Note on `postMessage`: how messages are to be composed is currently an unsolved question. The current general plan is to
mimic [Serde](https://serde.rs/) -like serialization, but that is in itself a substantial project. For the time being
it is recommended you lower to `web_sys` types and post plain `JsValue` objects.

## License

MIT
