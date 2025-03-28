use chrono::NaiveDate;

use crate::models::record_model::Record;

pub fn record_fixture(id: usize) -> Record {
    Record {
        id: id as i32,
        title: format!("title{}", id),
        artist: format!("artist{}", id),
        release_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap_or_default(),
        cover_url: format!("cover_url{}", id),
        discogs_url: Some(format!("discogs_url{}", id)),
        spotify_url: Some(format!("spotify_url{}", id)),
        user_id: 1,
        owned: true,
        wanted: false,
    }
}

pub fn records_fixture(num: usize) -> Vec<Record> {
    let mut records = vec![];
    for i in 1..num + 1 {
        records.push(record_fixture(i));
    }
    records
}
