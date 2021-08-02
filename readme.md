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
    - this can work for `Vec<T>`, `HashSet<T>`, `Option<T>`, or `T`.
- `#[Transform(<fooPath>)]` to mark which fields need a transformation through a unary function or named unary tuple (useful for `lift` values into `Option`);
- `#[MacroTransform(<macroPath>)]` to mark which fields will be wrapped inside a macro call (useful for `vec![]`ing single values);
- `#[From]` for `Enums`
- `#[Into]` for `Enums`:
  - marking the variant propagates the `#[Into]` to the subfields below
  - marking the single subfield is like marking a struct field
  - marking both causes an error for multiple occurrences of `#[Into]`
- `#[Transform]` and `#[MacroTransform]` for `Enums`:
  - marking the variant applies the transformation chain to the entire enum value
  - marking the single subfield is like marking a struct field
  - marking variant has precedence over marking the subfield
- Attribute order and number of occurrences matters:
  - `#[Into]` must be unique and the first one, if it occurs
  - `#[Transform]` and `#[MacroTransform]` can appear multiple times; the sooner they appear, the inner they result
    - for instance: `#[Transform(a)] #[Transform(b)] #[Transform(c)]` results in `c(b(a(<expr>)))`
    
## ToDo
- make `#[Into]` cover other standard examples;
- create a `#[Unhygienic(...)]` attribute for fields for unhygienic macro expression hacks;
- **way** better code aesthetics; `<-- in kinda progress`
- remove all metadata once the macro has finished its work. `<-- Can/Should I really do it?`

### Demo
Install `cargo-expand` and then:
```bash
cargo expand --test example
```
It outputs `tests/example.rs` expanded version. You will see what the macro generates.