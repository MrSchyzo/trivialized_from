use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(Input)]
struct Output {
    pub age: u8,
    #[Into]
    pub nested: NestedOutput,
}

#[derive(TrivializationReady)]
#[From(NestedInput)]
struct NestedOutput {
    pub field_0: u128,
}

struct Input {
    pub age: u8,
    pub nested: NestedInput,
}

struct NestedInput {
    pub field_0: u128,
}

#[allow(dead_code)]
fn main() {
    let _: Output = Input {
        age: 10u8,
        nested: NestedInput { field_0: 1024u128 },
    }
    .into();
}
