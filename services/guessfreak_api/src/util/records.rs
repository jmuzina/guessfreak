use std::error::Error;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn get_first_record_from_vec<TReturn: DeserializeOwned>(records: Vec<Value>) -> Option<TReturn> {
    if records.is_empty() {
        None
    } else {
        serde_json::from_value::<TReturn>(records[0].clone()).ok()
    }
}

pub fn get_first_record_from_result_vec<TReturn: DeserializeOwned, TResultErr>(records: Result<Vec<Value>, TResultErr>) -> Option<TReturn> {
    match records {
        Ok(records) => get_first_record_from_vec::<TReturn>(records),
        Err(_) => None
    }
}