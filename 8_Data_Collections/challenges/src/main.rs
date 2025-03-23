use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    let numbers: Vec<i32> = vec!(1, 4, 5, 5, 7, 4, 1, 3, 2, 9);
    let (median, mode) = median_and_mode(&numbers);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
    let sentence = "This is a randomly typed pig latin sentence to see the outcome of the function";
    let pig_latin = pig_latin_sentence(sentence);
    println!("Original: {}", sentence);
    println!("Pig Latin: {}", pig_latin);
    text_interface();
}

fn median_and_mode(numbers: &Vec<i32>) -> (f32, i32) {
    let mut sorted = numbers.clone();
    sorted.sort();
    // Calculate median:
    // For an odd number of elements, take the middle element.
    // (For an even number, you might average the two middle values.)
    let mid_index = sorted.len() / 2;
    let median = if sorted.len() % 2 == 1 {
        sorted[mid_index] as f32
    } else {    
        (sorted[mid_index - 1] as f32 + sorted[mid_index] as f32) / 2.0
    };
    // Calculate mode:
    // Create a hash map to count occurrences of each number.
    let mut counts = HashMap::new();
    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }
    
    // Find the number with the highest count.
    let mut mode = numbers[0];
    let mut max_count = 0;
    for (&num, &count) in counts.iter() {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }
    
    (median, mode)
}

fn pig_latin_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    // define the vowels (both lowercase and uppercase)
    let vowels = "aeiouAEIOU";

    if vowels.contains(first_char) {
        format!("{}hay ", word)
    } else {
        let first_char_len = first_char.len_utf8();
        let rest = &word[first_char_len..];
        format!("{}{}ay ", rest, first_char)
    }
}

fn pig_latin_sentence(sentence: &str) -> String {
    // split the sentence into words, convert each one, and join the sentence back together.
    sentence.split_whitespace()
    .map(|word| pig_latin_word(word))
    .collect::<Vec<_>>()
    .join("")
}

fn text_interface() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Company Employee Manager");
    println!("Commands:");
    println!("Add <Employee> to <Department>");
    println!("List <Department>    -- Lists employees in a department");
    println!("List all             -- Lists all departments and employees");
    println!("Exit");
    
    loop {
        // Prompt the user for a command.
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Please try again.");
            continue;
        }
        let input = input.trim();

        // Exit condition.
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Handle the "Add" command.
        if input.to_lowercase().starts_with("add ") {
            // Expected format: "Add <Employee> to <Department>"
            // We use simple tokenization. Note: This example assumes employee and department names are single words.
            let tokens: Vec<&str> = input.split_whitespace().collect();
            if tokens.len() < 4 || tokens[2].to_lowercase() != "to" {
                println!("Invalid format. Please use: Add <Employee> to <Department>");
                continue;
            }
            let employee = tokens[1].to_string();
            let department = tokens[3].to_string();

            // Add the employee to the department. If the department doesn't exist, create a new vector.
            company.entry(department.clone())
                   .or_insert(Vec::new())
                   .push(employee.clone());

            println!("Added {} to {}.", employee, department);
        }
        // Handle the "List" command.
        else if input.to_lowercase().starts_with("list ") {
            let tokens: Vec<&str> = input.split_whitespace().collect();
            if tokens.len() != 2 {
                println!("Invalid format. Use: List <Department> or List all");
                continue;
            }
            let arg = tokens[1].to_lowercase();
            if arg == "all" {
                // Get and sort departments.
                let mut departments: Vec<&String> = company.keys().collect();
                departments.sort();
                for dept in departments {
                    // Sort the employees alphabetically.
                    let mut employees = company.get(dept).unwrap().clone();
                    employees.sort();
                    println!("Department: {}", dept);
                    for employee in employees {
                        println!(" - {}", employee);
                    }
                }
            } else {
                // List a specific department.
                let department = tokens[1];
                if let Some(employees) = company.get(department) {
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    println!("Department: {}", department);
                    for employee in sorted_employees {
                        println!(" - {}", employee);
                    }
                } else {
                    println!("No such department: {}", department);
                }
            }
        }
        // Unknown command.
        else {
            println!("Unknown command. Please use 'Add', 'List', or 'Exit'.");
        }
    }

    println!("Exiting program.");
}