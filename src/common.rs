use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn test_fn<F>(input_file: &str, output_file: &str, f: F)
where
    F: Fn(String) -> String,
{
    // Read input from file
    let input = BufReader::new(File::open(input_file).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let input = input.join("\n");

    // Call the function to test
    let output = f(input);

    // Read expected output from file
    let expected_output = BufReader::new(File::open(output_file).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    // Compare output with expected output
    // println!("{}", &output);
    // println!("{}", &expected_output);
    assert_eq!(output, expected_output);
}
