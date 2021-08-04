use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(Input1)]
#[From(Input2)]
struct Output {
    pub field_1: u128,
    pub field_2: String,
}

struct Input1 {
    pub field_2: String,
    pub field_1: u128,
    pub field_0: Vec<String>,
}

struct Input2 {
    pub field_2: String,
    pub field_1: u128,
}

#[allow(dead_code)]
fn main() {
    let _: Output = Input1 {
        field_2: "".to_string(),
        field_1: 0u128,
        field_0: Vec::new(),
    }
    .into();

    let _: Output = Input2 {
        field_2: "".to_string(),
        field_1: 0u128,
    }
    .into();
}
