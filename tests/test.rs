use trivialized_from::TrivializationReady;

fn zerofy(_: u8) -> u8 {
    0
}

struct Wrapping {
    age: u8,
}

#[derive(TrivializationReady, PartialEq, Debug)]
#[From(Wrapping)]
struct DomainWrapping {
    #[MacroTransform(vec)]
    age: Vec<u8>,
}

pub enum Enum {
    Empty,
    WithSubRecord(SubRecord),
    WithAgeAndRecord(u8, SubRecord),
}

#[derive(TrivializationReady, Debug, PartialEq)]
#[From(Enum)]
pub enum DomainEnum {
    Empty,
    #[Into]
    WithSubRecord(SubRecord),
    WithAgeAndRecord(u8, #[Into] SubRecord),
}

#[derive(Debug, PartialEq)]
pub struct SubRecord {
    pub name: String,
}

pub struct Record {
    pub name: String,
    pub age: u8,
    pub maybe_record: Option<SubRecord>,
    pub records: Vec<SubRecord>,
    pub sub: SubRecord,
    pub e: Enum,
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
    #[Into]
    pub e: DomainEnum,
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let result: DomainRecord = Record {
            name: "Succ".to_owned(),
            age: 3u8,
            maybe_record: Some(SubRecord {
                name: "Secc".to_owned(),
            }),
            records: vec![SubRecord {
                name: "Sicc".to_owned(),
            }],
            sub: SubRecord {
                name: "Socc".to_owned(),
            },
            e: Enum::WithAgeAndRecord(
                5u8,
                SubRecord {
                    name: "Sacc".to_owned(),
                },
            ),
        }
        .into();

        let expected: DomainRecord = DomainRecord {
            name: "Succ".to_owned(),
            age: 0u8,
            maybe_record: Some(DomainSubRecord {
                name: "Secc".to_owned(),
            }),
            records: vec![DomainSubRecord {
                name: "Sicc".to_owned(),
            }],
            sub: DomainSubRecord {
                name: "Socc".to_owned(),
            },
            e: DomainEnum::WithAgeAndRecord(
                5u8,
                SubRecord {
                    name: "Sacc".to_owned(),
                },
            ),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn it_works_2() {
        let result: DomainWrapping = Wrapping { age: 3u8 }.into();

        let expected: DomainWrapping = DomainWrapping { age: vec![3u8] };

        assert_eq!(expected, result);
    }
}
