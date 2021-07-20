# Trivialized From

This is just a simple macro that handles specific cases of `From<_>` implementors.
It might reduce boilerplate under some circumstances.

## AsIs
- `#[derive(TrivializationReady)]` in order to enable the trivialized `From` implementation;
- `#[From(<TypePathCommaSeparatedList>)]` in order to set for what types the marked struct implements `From`;
    - it can be repeated and all types will be included in the `From` generation;
    - it tries to avoid duplicate types (it does only a `String.eq` comparison to keep uniqueness);
    - this only works for `struct`s;
    - TypePaths are expected to be "trivial" (no `&`);
- `#[Into]` to mark which fields need an `Into` conversion;
    - this can work for `Vec<T>`, `Option<T>`, or `T`.
- `#[Transform(<fooPath>)]` to mark which fields need a transformation through a unary function;
- make `#[From]` work for `Enums`
    - `#[Into]`, and `#[Transform]` are not implemented yet.
    
## ToDo
- implement `#[Into]` for enum variants and enum variant fields (semantic difference is not disclosed);
- implement `#[Transform]` for enum variants and enum variant fields (semantic difference is not disclosed);
- make `#[Into]` cover other standard examples;
- create a `#[Unhygienic(...)]` attribute for fields for unhygienic macro expression hacks;
- **way** better error handling (try to use `Span` and stuff);
- **way** better code aesthetics;
- remove all metadata once the macro has finished its work.

### Demo
Install `cargo-expand` and then:
```bash
cargo expand --test example
```
It outputs `tests/example.rs` expanded version. You will see what the macro generates.