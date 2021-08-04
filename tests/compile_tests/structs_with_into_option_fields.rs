use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(Input)]
struct Output {
    #[Into]
    pub nested: Option<NestedOutput>,
}

#[derive(TrivializationReady)]
#[From(NestedInput)]
struct NestedOutput {
    pub field_1: u128,
}

struct Input {
    pub nested: Option<NestedInput>,
}

struct NestedInput {
    pub field_1: u128,
}

#[allow(dead_code)]
fn main() {
    let _: Output = Input {
        nested: Some(NestedInput { field_1: 0 }),
    }
    .into();
}
