use csv::{ReaderBuilder, WriterBuilder};

use ::std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::Path;

use super::{NUM_STATS, NUM_TOTAL_STATS};

pub struct FileIO {
    file_path: String,
}

impl FileIO {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
    fn parse_total_data_from_str(str: &str) -> [u32; NUM_TOTAL_STATS] {
        let split_data = str
            .split(",")
            .filter_map(|value| value.trim().parse().ok())
            .collect::<Vec<u32>>();
        let mut total_stats = [0; NUM_TOTAL_STATS];
        for i in 0..NUM_TOTAL_STATS {
            total_stats[i] = split_data[i];
        }

        return total_stats;
    }
    fn parse_data_from_str(str: &str) -> Option<(String, [u32; NUM_STATS])> {
        let mut split_data = str.split(",");
        if let Some(hand) = split_data.next() {
            let mut counts = [0; NUM_STATS];
            let parsed_counts = split_data
                .filter_map(|count_str| count_str.trim().parse().ok())
                .collect::<Vec<u32>>();
            for i in 0..NUM_STATS {
                counts[i] = parsed_counts[i];
            }
            return Some((hand.trim().to_string(), counts));
        }
        return None;
    }
    pub fn read_from_file(
        &self,
    ) -> Result<(HashMap<String, [u32; NUM_STATS]>, [u32; NUM_TOTAL_STATS]), Box<dyn Error>> {
        if !Path::new(&self.file_path).exists() {
            return Ok((HashMap::new(), [0; NUM_TOTAL_STATS]));
        }
        let file = File::open(&self.file_path)?;
        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
        let mut response_data = HashMap::new();

        let total_stats_record = match reader.records().next() {
            Some(total_stats_result) => total_stats_result?,
            None => return Err("Error parsing".into()),
        };
        let total_stats_str = &total_stats_record[0];
        let total_stats = FileIO::parse_total_data_from_str(total_stats_str);
        for result in reader.records() {
            let record = result?;
            let record_as_str = &record[0];
            if let Some((hand, counts)) = FileIO::parse_data_from_str(record_as_str) {
                response_data.insert(hand, counts);
            } else {
                return Err("Error parsing".into());
            }
        }
        return Ok((response_data, total_stats));
    }
    pub fn write_to_file(&self, line: String) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        let mut writer = WriterBuilder::new().from_writer(file);
        writer.write_record(&[line])?;
        writer.flush()?;
        return Ok(());
    }
    pub fn clear_file(&self) -> Result<(), Box<dyn Error>> {
        File::create(&self.file_path)?;
        return Ok(());
    }
    pub fn array_to_string<T: ToString>(array: &[T]) -> String {
        return array
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(",");
    }
}
