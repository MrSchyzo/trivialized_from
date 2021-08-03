use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(Input)]
struct Output {
    pub field_1: u128,
    pub field_2: String,
}

struct Input {
    pub field_0: Option<i8>,
    pub field_2: String,
    pub field_1: u128,
}

#[allow(dead_code)]
fn main() {
    let _: Output = Input {
        field_0: None,
        field_2: "".to_string(),
        field_1: 0u128,
    }
    .into();
}
