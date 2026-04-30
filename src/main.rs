use polars::prelude::*;
use std::env;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_start = Instant::now(); // Start benchmarking timer

    // Get Command Line Arguments
    let target_path = env::args().nth(1);
    let filtering_path = env::args().nth(2);
    let filtering_column = env::args().nth(3);
    let filtered_file = env::args().nth(4);
    let mut filtering_vector: Vec<String> = vec![]; // create vector to hold filtering lines

    match target_path {
        Some(target_arg) => { // If there is a target file
            println!("Target-Language File Present... {:?}", target_arg);
            // Read in the target file
            let target_tsv = LazyCsvReader::new(PlRefPath::new(target_arg))
                .with_separator(b'\t') // Set the delimiter to tabs rather than commas
                .with_has_header(true) // Columns have names
                .finish()?
                .collect()?;

            match filtering_path {
                Some(english_arg) => {
                    println!("Filtering File Present... {:?}", english_arg);
                    // Push each line from the filtering file to the filtering vector
                    for line in std::fs::read_to_string(&english_arg)?.lines() {
                        filtering_vector.push(line.to_string());
                    }

                    match filtering_column {
                        Some(column) => { // If there is a column to filter on
                            // Create a series from the filtering vector
                            let filter_series =
                                Series::new(column.clone().into(), &filtering_vector);
                            // Filter the target table to only include the rows containig the values
                            // provided in the filtering file
                            let mut filtered_table = target_tsv
                                .clone()
                                .lazy()
                                .filter(col(column.clone()).is_in(lit(filter_series), false))
                                .collect()
                                .unwrap();

                            match filtered_file {
                                Some(output) => { // If an output file is provided
                                    println!("Output File Name Present... {:?}", output);
                                    // Write the filtered table to the output
                                    let mut out_file = fs::File::create(output).unwrap();
                                    CsvWriter::new(&mut out_file)
                                        .with_separator(b'\t') // Set the delimiter to tab
                                        .finish(&mut filtered_table)
                                        .unwrap();
                                }
                                None => {
                                    println!("[ ERROR ] - No Output File Provided (Argument 4)")
                                }
                            }
                        }
                        None => println!("[ ERROR ] - No Filtering Column Provided (Argument 3)"),
                    }
                }
                None => println!("[ ERROR ] - No Filtering File Provided (Argument 2)"),
            }
        }
        None => println!("[ ERROR ] - No Primary File Provided (Argument 1)"),
    }

    let program_time = program_start.elapsed(); // End benchmarking timer
    println!( // Print the benchmarking time to the terminal
        "{} Lines Filtered in {:.2?}",
        filtering_vector.len(),
        program_time
    );

    Ok(())
}
