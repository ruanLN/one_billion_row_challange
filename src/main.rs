use std::fmt::{Display, Formatter};
use std::{fs::File, path::Path};
use std::io::{BufReader, stdout, BufWriter};
use std::io::prelude::*;

use rustc_hash::FxHashMap;

#[derive(Debug)]
struct StationData {
    min: f32,
    max: f32,
    total: f32,
    count: f32,
}

impl Default for StationData {
    fn default() -> Self {
        StationData {
            min: f32::MAX,
            max: f32::MIN,
            total: 0.0,
            count: 0.0,
        }
    }
}

impl Display for StationData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "{:.1}/{:.1}/{:.1}", self.min, self.mean(), self.max)
    }
}

impl StationData {
    fn update(&mut self, value: f32) {
        self.count += 1.0;
        self.total += value;
        self.max = self.max.max(value);
        self.min = self.min.min(value);
    }

    fn mean(&self) -> f32 {
        self.total / self.count
    }
 }
fn main() -> Result<(),  Box<dyn std::error::Error>> {
    let file = File::open(Path::new("./measurements_1_000_000_000.txt"))?;
    let mut buf_reader = BufReader::new(file);
    let mut station_data = FxHashMap::default();
    station_data.reserve(500);
    let mut buffer = String::with_capacity(120);
    while buf_reader.read_line(&mut buffer)? != 0 {
        let line = &buffer[..buffer.len() - 1];
        let sep_index = line.find(';').unwrap();
        let city = &line[..sep_index];
        let temp = &line[sep_index+1..];
        let temp: f32 = fast_float::parse(temp).unwrap();
        if !station_data.contains_key(city) {
            station_data.insert(city.to_owned(), StationData::default());
        }
        station_data.get_mut(city).unwrap().update(temp);
        buffer.clear();
    }
    let mut station_names: Vec<_> = station_data.keys().into_iter().collect();
    station_names.sort();
    let mut stream = BufWriter::new(stdout());
    write!(stream, "{{")?;
    for station in station_names {
        write!(stream, "{station}={}", station_data[station])?;
    }
    write!(stream, "}}")?;
    return Ok(());
}
