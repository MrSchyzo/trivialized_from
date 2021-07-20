use trivialized_from::TrivializationReady;

#[allow(unused)]
enum ExampleEnum {
    Empty,
    WithSubRecord(ExampleSubRecord),
    WithAgeAndRecord(u8, ExampleSubRecord),
}

#[allow(unused)]
#[derive(TrivializationReady)]
#[From(ExampleEnum)]
enum ExampleDomainEnum {
    Empty,
    #[Into]
    WithSubRecord(ExampleSubRecord),
    WithAgeAndRecord(u8, #[Into] ExampleSubRecord),
}

#[allow(unused)]
struct ExampleSubRecord {
    pub name: String,
}

#[allow(unused)]
struct ExampleRecord {
    pub name: String,
    pub age: u8,
    pub maybe_record: Option<ExampleSubRecord>,
    pub records: Vec<ExampleSubRecord>,
    pub sub: ExampleSubRecord,
    pub e: ExampleEnum,
}

#[allow(unused)]
#[derive(TrivializationReady)]
#[From(ExampleRecord)]
#[From(ExampleSubRecord)]
struct ExampleDomainSubRecord {
    pub name: String,
}

#[allow(unused)]
#[derive(TrivializationReady)]
#[From(ExampleRecord)]
struct ExampleDomainRecord {
    pub name: String,
    #[Transform(example_zerofy)]
    pub age: u8,
    #[Into]
    pub maybe_record: Option<ExampleDomainSubRecord>,
    #[Into]
    pub records: Vec<ExampleDomainSubRecord>,
    #[Into]
    pub sub: ExampleDomainSubRecord,
    #[Into]
    pub e: ExampleDomainEnum,
}

#[allow(unused)]
fn example_zerofy(_: u8) -> u8 {
    0
}
