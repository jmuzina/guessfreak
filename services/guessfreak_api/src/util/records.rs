use std::error::Error;
use std::fmt::Display;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn get_first_record_from_vec<TReturn: DeserializeOwned>(records: Vec<Value>) -> Option<TReturn> {
    if records.is_empty() {
        None
    } else {
        match serde_json::from_value::<TReturn>(records[0].clone()).ok() {
            Some(record) => Some(record),
            None => {
                log::error!("Failed to serialize record");
                None
            }
        }
    }
}

pub fn get_first_record_from_result_vec<TReturn: DeserializeOwned, TResultErr: Display>(records: Result<Vec<Value>, TResultErr>) -> Option<TReturn> {
    match records {
        Ok(records) => get_first_record_from_vec::<TReturn>(records),
        Err(e) => {
            log::error!("Failed to get first record from records due to error {}", e);
            None
        }
    }
}