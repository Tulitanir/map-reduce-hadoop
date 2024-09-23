pub mod cases_deaths;
pub mod utils;

use std::{collections::HashMap, io::{self, BufRead}};
use cases_deaths::CasesDeaths;
use utils::correlation;

fn main() {
    let stdin = io::stdin();
    let mut country_data: HashMap<String, Vec<CasesDeaths>> = HashMap::new();

    for line in stdin.lock().lines() {
        if let Ok(line_str) = line {
            let parts: Vec<&str> = line_str.split('\t').collect();
            if parts.len() == 2 {
                let country = parts[0].to_string();
                let values: Vec<f64> = parts[1]
                    .split(',')
                    .filter_map(|s| s.parse::<f64>().ok())
                    .collect();
                
                // Ожидаем, что в values: [stringency_index, new_cases, new_deaths]
                if values.len() == 3 {
                    let (stringency_index, new_cases, new_deaths) = (values[0], values[1], values[2]);
                    
                    country_data.entry(country).or_insert(Vec::new()).push(
                        CasesDeaths{
                            stringency_index, 
                            new_cases, 
                            new_deaths
                        }
                    );
                }
            }
        }
    }

    // Вычисляем корреляции
    for (country, data) in country_data {
        let stringency_index: Vec<f64> = data.iter().map(|cases| cases.stringency_index).collect();
        let new_cases: Vec<f64> = data.iter().map(|cases| cases.new_cases).collect();
        let new_deaths: Vec<f64> = data.iter().map(|cases| cases.new_deaths).collect();

        // Пропускаем, если данных слишком мало
        if stringency_index.len() <= 1 || new_cases.len() <= 1 {
            println!("{}\tНедостаточно данных для корреляции", country);
            continue;
        }

        let correlation_cases = correlation(&stringency_index, &new_cases);
        let correlation_deaths = correlation(&stringency_index, &new_deaths);

        // Выводим страну и корреляции
        println!(
            "{}\tКорреляция со случаями заражения: {}\tКорреляция со смертями: {}",
            country,
            correlation_cases.unwrap_or(0.0),
            correlation_deaths.unwrap_or(0.0)
        );
    }
}
