use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(Input)]
struct Output {
    #[Transform(u8_to_string)]
    #[MacroTransform(vec)]
    #[Transform(Option::Some)]
    pub age_1: Option<Vec<String>>,
}

struct Input {
    pub age_1: u8,
}

fn u8_to_string(a: u8) -> String {
    a.to_string()
}

#[allow(dead_code)]
fn main() {
    let _: Output = Input { age_1: 5u8 }.into();
}
