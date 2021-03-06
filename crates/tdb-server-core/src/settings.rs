use std::env;
use std::error::Error;
use std::str::FromStr;

pub fn key_or_default_parse<
    E: Into<Box<dyn Error>>,
    T: FromStr<Err=E>
>(key: &str, default: T) -> Result<T, Box<dyn Error>> {
    match env::var(key) {
        Ok(val) => val.parse::<T>().map_err(|err| err.into()),
        Err(_) => Ok(default),
    }
}

pub fn key_or_default(key: &str, default: &str) -> String {
   match env::var(key) {
        Ok(val) => val,
        Err(_) => default.into(),
    }
}

pub fn key_or_none(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

#[derive(Clone, Debug, Default)]
pub struct Settings {
    /// autoflush: boolean. Flush everything to disk at some interval.
    pub autoflush: bool,
    /// dtf_folder: string. folder to save .dtf files
    pub dtf_folder: String,
    /// flush_interval: u32. flush at some regular interval.
    pub flush_interval: u32,
    /// record count history every 3 seconds
    pub granularity: u64,
    /// history circular queue capacity
    pub q_capacity: usize,
    /// settings for influxdb
    pub influx: Option<InfluxSettings>,
}

#[derive(Clone, Debug, Default)]
pub struct InfluxSettings {
    pub host: String,
    pub db: String,
    pub interval: u64,
}