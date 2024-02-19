Minimal reproduction of a bug(?) with wit-bindgen.

Running `cargo component builcd --release` on this repo fails with:

```
  = note: rust-lld: error: duplicate symbol: cabi_realloc
          >>> defined in /Volumes/workplace/wit-bindgen-symbol-repro/target/wasm32-wasi/release/deps/libwit_bindgen-796469586291d0cd.rlib(wit_bindgen-796469586291d0cd.wit_bindgen.3ccd0498004a9c9c-cgu.0.rcgu.o)
          >>> defined in /Volumes/workplace/wit-bindgen-symbol-repro/target/wasm32-wasi/release/deps/libwit_bindgen-fb8c62ead352e205.rlib(wit_bindgen-fb8c62ead352e205.wit_bindgen.15338958f611d1dd-cgu.0.rcgu.o)
```

Downgrading the `wit-bindgen` dependency in Cargo.toml to `0.16.0` allows the project to build again. This seems to be because the `wasi` crate's bindings are built with `wit-bindgen = 0.16.0` and the multiple versions create conflicting symbols.
