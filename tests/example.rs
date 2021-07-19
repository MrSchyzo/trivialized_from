use trivialized_from::TrivializationReady;

pub struct SubRecord {
    pub name: String,
}

pub struct Record {
    pub name: String,
    pub age: u8,
    pub maybe_record: Option<SubRecord>,
    pub records: Vec<SubRecord>,
    pub sub: SubRecord,
}

#[derive(TrivializationReady)]
#[From(Record)]
#[From(SubRecord)]
pub struct DomainSubRecord {
    pub name: String,
}

#[derive(TrivializationReady)]
#[From(Record)]
pub struct DomainRecord {
    pub name: String,
    #[Transform(zerofy)]
    pub age: u8,
    #[Into]
    pub maybe_record: Option<DomainSubRecord>,
    #[Into]
    pub records: Vec<DomainSubRecord>,
    #[Into]
    pub sub: DomainSubRecord,
}

fn zerofy(_: u8) -> u8 {
    0
}