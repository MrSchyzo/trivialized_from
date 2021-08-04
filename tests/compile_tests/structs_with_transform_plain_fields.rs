use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(Input)]
struct Output {
    #[Transform(ToString::to_string)]
    pub age: String,
    #[Transform(u8_to_string)]
    pub age_1: String,
}

struct Input {
    pub age: &'static u8,
    pub age_1: u8,
}

fn u8_to_string(a: u8) -> String {
    a.to_string()
}

#[allow(dead_code)]
fn main() {
    let _: Output = Input {
        age: &10u8,
        age_1: 5u8,
    }
    .into();
}
