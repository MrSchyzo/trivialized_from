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
- `#[Into]` to mark what fields need an `Into` conversion;
    - this can work for `Vec<T>`, `Option<T>`, or `T`.
    
## ToDo
- make `#[From]` work for `Unions` and `Enums`;
- create a `#[Transform(...)]` attribute for fields for calling custom functions/expressions;
- make `#[Into]` cover other standard examples;
- create a `#[Unhygienic(...)]` attribute for fields for unhygienic macro expression hacks;
- better error handling (try to use `Span` and stuff).