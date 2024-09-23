pub mod values;

use std::io::{self, BufRead};

use values::Values;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(record_str) = line {
            if record_str.starts_with("iso_code") {
               continue;
            }

            let values: Vec<&str> = record_str.split(',').collect();
            let location = values[Values::Location as usize];

            if let (
                Ok(stringency_index),
                Ok(new_cases),
                Ok(new_deaths)
            ) = (
                values[Values::StringencyIndex as usize].parse::<f64>(),
                values[Values::NewCasesSmoothedPerMillion as usize].parse::<f64>(),
                values[Values::NewDeathsSmoothedPerMillion as usize].parse::<f64>()
            ) {
                // Формируем ключ-значение для вывода
                println!(
                    "{}\t{},{},{}",
                    location,
                    stringency_index,
                    new_cases,
                    new_deaths
                );
            }
        }
    }
}
