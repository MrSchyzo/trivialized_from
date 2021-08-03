use trivialized_from::TrivializationReady;

#[derive(TrivializationReady)]
#[From(B)]
struct A;

struct B;

fn main() {
    let _: A = B {}.into();
}
