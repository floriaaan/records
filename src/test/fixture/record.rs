use crate::models::record_model::Record;

pub fn record_fixture(id: usize) -> Record {
    Record {
        id: id as i32,
        name: String::from("apple"),
    }
}

pub fn records_fixture(num: usize) -> Vec<Record> {
    let mut records = vec![];
    for i in 1..num + 1 {
        records.push(record_fixture(i));
    }
    records
}
