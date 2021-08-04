use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(Input)]
struct Output {
    #[MacroTransform(vec)]
    pub age: Vec<u8>,
}

struct Input {
    pub age: u8,
}

#[allow(dead_code)]
fn main() {
    let _: Output = Input { age: 10u8 }.into();
}
