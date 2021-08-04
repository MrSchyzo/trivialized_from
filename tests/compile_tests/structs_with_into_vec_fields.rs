use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(Input)]
struct Output {
    #[Into]
    pub nested: Vec<NestedOutput>,
}

#[derive(TrivializationReady)]
#[From(NestedInput)]
struct NestedOutput {
    pub field_1: u128,
}

struct Input {
    pub nested: Vec<NestedInput>,
}

struct NestedInput {
    pub field_1: u128,
}

#[allow(dead_code)]
fn main() {
    let _: Output = Input {
        nested: vec![NestedInput { field_1: 0 }],
    }
    .into();
}
