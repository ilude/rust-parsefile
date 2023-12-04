use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Target {
    commands: Vec<String>,
}

impl Target {
    fn new(commands: Vec<String>) -> Self {
        Target { commands }
    }
}

fn parse_justfile(content: &str) -> HashMap<String, Target> {
    let mut rules = HashMap::new();

    let lines = content.lines();

    let mut current_rule_name = String::new();
    let mut current_commands = Vec::new();

    for line in lines {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            // Skip empty lines and comments
            continue;
        }

        if let Some(colon_index) = trimmed_line.find(':') {
            // New rule
            if !current_rule_name.is_empty() {
                // Save the previous rule
                let target = Target::new(current_commands.clone());
                rules.insert(current_rule_name.clone(), target);
                current_commands.clear();
            }

            // Extract rule name
            current_rule_name = trimmed_line[0..colon_index].trim().to_string();
        } else {
            // Command for the current rule
            current_commands.push(trimmed_line.to_string());
        }
    }

    // Save the last rule
    if !current_rule_name.is_empty() {
        let target = Target::new(current_commands.clone());
        rules.insert(current_rule_name, target);
    }

    rules
}

fn main() {
    // Read the Justfile content
    let justfile_content = fs::read_to_string("Justfile").expect("Failed to read Justfile");

    // Parse the Justfile
    let target = parse_justfile(&justfile_content);

    // Print the parsed rules
    for (name, target) in target {
        println!("Target: {}", name);
        for command in target.commands {
            println!("  - {}", command);
        }
        println!();
    }
}
