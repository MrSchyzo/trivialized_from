use trivialized_from::TrivializationReady;

fn zerofy(some: u8) -> u8 {
    0
}

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

#[derive(TrivializationReady, PartialEq, Debug)]
#[From(Record)]
#[From(SubRecord)]
pub struct DomainSubRecord {
    pub name: String,
}

#[derive(TrivializationReady, PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let result: DomainRecord = Record {
            name: "Succ".to_owned(),
            age: 3u8,
            maybe_record: Some(SubRecord{name: "Secc".to_owned()}),
            records: vec![SubRecord{name: "Sicc".to_owned()}],
            sub: SubRecord{name: "Socc".to_owned()},
        }.into();

        let expected: DomainRecord = DomainRecord {
            name: "Succ".to_owned(),
            age: 0u8,
            maybe_record: Some(DomainSubRecord{name: "Secc".to_owned()}),
            records: vec![DomainSubRecord{name: "Sicc".to_owned()}],
            sub: DomainSubRecord{name: "Socc".to_owned()},
        };

        assert_eq!(expected, result);
    }
}