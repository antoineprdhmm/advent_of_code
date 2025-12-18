use crate::read_input;

// just split lines by whitespace to get the values here
fn read_data_part_1() -> (Vec<Vec<usize>>, Vec<String>) {
    let lines = read_input(2025, 6)
        .unwrap()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let numbers: Vec<Vec<usize>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let operations: Vec<String> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect();

    (numbers, operations)
}

pub fn run_part_1() {
    let (numbers, operations) = read_data_part_1();

    let total_sum = operations
        .iter()
        .enumerate()
        .map(|(i, operation)| {
            // check operation kind
            // then apply operation on column numbers
            match operation.as_str() {
                "*" => numbers.iter().map(|row| row[i]).product::<usize>(),
                "+" => numbers.iter().map(|row| row[i]).sum::<usize>(),
                op => panic!("Unknown operation: {op}"),
            }
        })
        .sum::<usize>();

    assert_eq!(5873191732773, total_sum);
}

fn read_data_part_2() -> (Vec<Vec<usize>>, Vec<String>) {
    let lines: Vec<Vec<char>> = read_input(2025, 6)
        .unwrap()
        .map_while(Result::ok)
        .map(|s| s.chars().collect())
        .collect();

    // here we must keep the whitespaces around the values
    // let's find the columns with only whitespace -> used to separate the values of each operation

    let separators_cols: Vec<usize> = (0..lines[0].len())
        .filter(|&col| lines.iter().all(|line| line[col] == ' '))
        .collect();

    // now, that we know the separator columns, we can split and get the values for each operation

    let mut splitted_lines = Vec::new();

    for line in &lines {
        let mut chunks = Vec::new();

        let mut prev = 0;
        for end in &separators_cols {
            let chunk: String = line[prev..*end].iter().collect();
            chunks.push(chunk);
            prev = end + 1;
        }

        let chunk: String = line[prev..].iter().collect();
        chunks.push(chunk);

        splitted_lines.push(chunks);
    }

    // To get the operations -> same as part 1

    let operations: Vec<String> = splitted_lines
        .last()
        .unwrap()
        .clone()
        .iter()
        .map(|r| r.trim().to_string())
        .collect();

    // Instead of having values by line, let's store the values by column
    // Will be easier

    let mut operations_values = Vec::new();

    for operation_idx in 0..operations.len() {
        let mut operation_value = Vec::new();
        // go through all lines except last because it's the operation
        for line_idx in 0..(splitted_lines.len() - 1) {
            operation_value.push(splitted_lines[line_idx][operation_idx].clone());
        }
        operations_values.push(operation_value);
    }

    // Now let's read the numbers for each operation as columns

    let mut col_operations_values = Vec::new();
    for operation_values in operations_values {
        let mut values = Vec::new();

        // All strings values for an operation have the same len (including the potential whitespaces around)
        let value_len = operation_values[0].len();

        // Better perfs than nth()
        let operation_values: Vec<Vec<char>> = operation_values
            .iter()
            .map(|s| s.chars().collect())
            .collect();

        // each col is a potential number vertically
        for col_idx in 0..value_len {
            // we first build it as string
            let mut value_str = String::new();
            for chars in &operation_values {
                value_str.push(chars[col_idx]);
            }
            // then trim whitespaces and convert to int
            values.push(value_str.trim().parse::<usize>().unwrap());
        }

        col_operations_values.push(values);
    }

    (col_operations_values, operations)
}

pub fn run_part_2() {
    let (numbers, operations) = read_data_part_2();

    let total_sum: usize = operations
        .iter()
        .enumerate()
        .map(|(i, operation)| {
            let operation_numbers = &numbers[i];

            match operation.as_str() {
                "*" => operation_numbers.iter().product::<usize>(),
                "+" => operation_numbers.iter().sum::<usize>(),
                op => panic!("Unknown operation: {op}"),
            }
        })
        .sum();

    assert_eq!(total_sum, 11386445308378);
}
