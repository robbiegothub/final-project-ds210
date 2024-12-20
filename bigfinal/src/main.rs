
use std::error::Error;
use std::fs::File;
use std::path::Path;
mod main_environment{
    use std::error::Error;
    use std::fs::File;
    use std::path::Path;



    pub fn read_csv_to_vector(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let file = File::open(Path::new(file_path))?;
        let mut reader = csv::Reader::from_reader(file);
        let mut rows: Vec<Vec<String>> = Vec::new();
        for result in reader.records() {
            let record = result?; // Each record is a CSV row
            rows.push(record.iter().map(|s| s.to_string()).collect());
        }
        Ok(rows)
    
    }

    pub fn filter_by_countries(
        data: &[Vec<String>],
        countries: &[&str],
    ) -> Vec<Vec<String>> {
        let mut filtered_rows: Vec<Vec<String>> = Vec::new();
        for row in data {
            if let Some(second_entry) = row.get(1) { 
                if countries.contains(&second_entry.as_str()) { 
                     filtered_rows.push(row.clone()); 
                }
            }
        }
            filtered_rows
    }



    pub fn filter_by_country(
        data: &[Vec<String>],
        country: &str,
    ) -> Vec<Vec<String>> {
        data.iter()
            .filter(|row| {
                let is_target_country = row.get(1).map_or(false, |entry| entry == country);
                let is_correct_description = row.get(3).map_or(false, |entry| entry == "International migrant stock: Both sexes (number)");
                is_target_country && is_correct_description
            })
            .cloned()
            .collect()
    }

    pub fn comparing_immigration_numbers(
        filtered_rows: Vec<(String, Vec<Vec<String>>)>, 
        years: &[&str],                               
    ) -> Vec<(String, Vec<(i64, i64, i64,f64, String)>)> {
        filtered_rows
            .into_iter()
            .map(|(country_name, rows)| {
                let comparisons = rows
                    .windows(2) 
                    .enumerate() 
                    .filter_map(|(i, pair)| {
                        if let (Some(first), Some(second)) = (pair.get(0), pair.get(1)) {
                            let value1 = first.get(4).cloned().unwrap_or_default().replace(",", "");
                            let value2 = second.get(4).cloned().unwrap_or_default().replace(",", "");
    
                            
                            if let (Ok(num1), Ok(num2)) = (value1.parse::<i64>(), value2.parse::<i64>()) {
                                let percentage_diff = ((num2 as f64 - num1 as f64) / num1 as f64) * 100.0;
                                let percentage_diff_rounded = (percentage_diff * 100.0).round() / 100.0;
                                let year_range = years.get(i).unwrap_or(&"Unknown Range").to_string();
    
                              
                                return Some((num1, num2, num2 - num1,percentage_diff_rounded, year_range));
                            }
                        }
                        None
                    })
                    .collect();
    
                (country_name, comparisons)
            })
            .collect()
        }

    
}

#[cfg(test)]
    mod tests {
        use super::*;

    #[test]
        fn test_comparing_immigration_numbers () {
            let file_path = "SYB67_327_202411_International Migrants and Refugees.csv";
            use crate::main_environment::read_csv_to_vector;
            use crate::main_environment::filter_by_country;
            use crate::main_environment::comparing_immigration_numbers;
            use crate::main_environment::filter_by_countries;
            match read_csv_to_vector(file_path) {
                Ok(rows) => {
                    let target_countries = [
                        "Russian Federation",
                        "Ukraine",
                        "Syrian Arab Republic",
                        "Venezuela",
                        "Yemen",
                        "United States of America",
                        "Türkiye",
                        "Saudi Arabia",
                        "China",
                        "Australia",
                        
                    ];
                    let filtered_rows = filter_by_countries(&rows, &target_countries);
                    let filtered_row_russia = filter_by_country(&rows, "Russian Federation");
                    let filtered_row_ukraine = filter_by_country(&rows, "Ukraine");
                    let filtered_row_syria = filter_by_country(&rows, "Syrian Arab Republic");
                    let filtered_row_venezuela = filter_by_country(&rows, "Venezuela (Boliv. Rep. of)");
                    let filtered_row_yemen = filter_by_country(&rows, "Yemen");
                    let filtered_row_us = filter_by_country(&rows,"United States of America");
                    let filtered_row_turkey = filter_by_country(&rows,"Türkiye");
                    let filtered_row_saudi = filter_by_country(&rows,"Saudi Arabia");
                    let filtered_row_china = filter_by_country(&rows,"China");
                    let filtered_row_aus = filter_by_country(&rows,"Australia");
                    let test_filtered_vecs = vec![
                            ("Russian Federation".to_string(),filtered_row_russia),
                            ("Ukraine".to_string(),filtered_row_ukraine),
                            ("Syrian Arab Republic".to_string(),filtered_row_syria),
                            ("Venezuela".to_string(),filtered_row_venezuela),
                            ("Yemen".to_string(),filtered_row_yemen),
                            ("United States of America".to_string(),filtered_row_us),
                            ("Türkiye".to_string(),filtered_row_turkey),
                            ("Saudi Arabia".to_string(),filtered_row_saudi),
                            ("China".to_string(),filtered_row_china),
                            ("Australia".to_string(),filtered_row_aus),
                            
                        ];
        
                    let years = [
                        "2005->2010",
                        "2010->2015",
                        "2015->2020",
                    ];
                    let testcomparisons = comparing_immigration_numbers(test_filtered_vecs, &years);
        
                    for (country_name, country_comparisons) in testcomparisons {
                        for (num1, num2, _, _, _) in country_comparisons {
                        
                            assert!(
                                num1 > 0,
                                "Found non-positive num1 in {}: {}",
                                country_name,
                                num1
                        );
                            assert!(
                                num2 > 0,
                                "Found non-positive num2 in {}: {}",
                                country_name,
                                num2
                            );
                        }
                    }
                }
                Err(e) => eprintln!("Error reading CSV: {}", e),
                }
            
            
           
    }

    #[test]
fn test_filtered_rows_not_empty() {
    let file_path = "SYB67_327_202411_International Migrants and Refugees.csv";
    use crate::main_environment::read_csv_to_vector;
    use crate::main_environment::filter_by_country;

    match read_csv_to_vector(file_path) {
        Ok(rows) => {
            let filtered_row_russia = filter_by_country(&rows, "Russian Federation");
            let filtered_row_ukraine = filter_by_country(&rows, "Ukraine");
            let filtered_row_syria = filter_by_country(&rows, "Syrian Arab Republic");
            let filtered_row_venezuela = filter_by_country(&rows, "Venezuela (Boliv. Rep. of)");
            let filtered_row_yemen = filter_by_country(&rows, "Yemen");
            let filtered_row_us = filter_by_country(&rows, "United States of America");
            let filtered_row_turkey = filter_by_country(&rows, "Türkiye");
            let filtered_row_saudi = filter_by_country(&rows, "Saudi Arabia");
            let filtered_row_china = filter_by_country(&rows, "China");
            let filtered_row_aus = filter_by_country(&rows, "Australia");

            let filtered_rows = vec![
                ("Russian Federation", filtered_row_russia),
                ("Ukraine", filtered_row_ukraine),
                ("Syrian Arab Republic", filtered_row_syria),
                ("Venezuela", filtered_row_venezuela),
                ("Yemen", filtered_row_yemen),
                ("United States of America", filtered_row_us),
                ("Türkiye", filtered_row_turkey),
                ("Saudi Arabia", filtered_row_saudi),
                ("China", filtered_row_china),
                ("Australia", filtered_row_aus),
            ];

            for (country_name, filtered_row) in filtered_rows {
                assert!(
                    !filtered_row.is_empty(),
                    "Filtered rows for {} are empty!",
                    country_name
                );
            }
        }
        Err(e) => eprintln!("Error reading CSV: {}", e),
    }
}

    }
fn main() {
    let file_path = "SYB67_327_202411_International Migrants and Refugees.csv";
    use main_environment::read_csv_to_vector;
    use main_environment::filter_by_countries;
    use main_environment::filter_by_country;
    use main_environment::comparing_immigration_numbers;
    match read_csv_to_vector(file_path) {
        Ok(rows) => {
            let target_countries = [
                "Russian Federation",
                "Ukraine",
                "Syrian Arab Republic",
                "Venezuela",
                "Yemen",
                "United States of America",
                "Türkiye",
                "Saudi Arabia",
                "China",
                "Australia",
                
            ];
            let filtered_rows = filter_by_countries(&rows, &target_countries);
            let filtered_row_russia = filter_by_country(&rows, "Russian Federation");
            let filtered_row_ukraine = filter_by_country(&rows, "Ukraine");
            let filtered_row_syria = filter_by_country(&rows, "Syrian Arab Republic");
            let filtered_row_venezuela = filter_by_country(&rows, "Venezuela (Boliv. Rep. of)");
            let filtered_row_yemen = filter_by_country(&rows, "Yemen");
            let filtered_row_us = filter_by_country(&rows,"United States of America");
            let filtered_row_turkey = filter_by_country(&rows,"Türkiye");
            let filtered_row_saudi = filter_by_country(&rows,"Saudi Arabia");
            let filtered_row_china = filter_by_country(&rows,"China");
            let filtered_row_aus = filter_by_country(&rows,"Australia");
            
            let filtered_vecs = vec![
                ("Russian Federation".to_string(),filtered_row_russia),
                ("Ukraine".to_string(),filtered_row_ukraine),
                ("Syrian Arab Republic".to_string(),filtered_row_syria),
                ("Venezuela".to_string(),filtered_row_venezuela),
                ("Yemen".to_string(),filtered_row_yemen),
                ("United States of America".to_string(),filtered_row_us),
                ("Türkiye".to_string(),filtered_row_turkey),
                ("Saudi Arabia".to_string(),filtered_row_saudi),
                ("China".to_string(),filtered_row_china),
                ("Australia".to_string(),filtered_row_aus),
                    
            ];
            let years = [
                "2005->2010",
                "2010->2015",
                "2015->2020",
            ];
            let comparisons = comparing_immigration_numbers(filtered_vecs, &years);
            for (country_name, country_comparisons) in comparisons {
                println!("{}", country_name);
                for (num1, num2, difference, percentage_diff_rounded, year_range) in country_comparisons {
                    println!("{} -> {} (Difference: {}({}%)) \"{}\"", num1, num2, difference,percentage_diff_rounded, year_range);
                }
                println!();
            }
            

           
        }
        Err(e) => eprintln!("Error reading CSV: {}", e),

    }
}


