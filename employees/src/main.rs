use std::{collections::HashMap, io::stdin};

fn print_header() {
    println!(
        "\
EMPLOYEE DIRECTORY
==================

Add employee       :: add NAME to DEPARTMENT
List all employees :: list all
List department    :: list DEPARTMENT
Quit directory     :: quit
"
    );
}

fn main() {
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    let mut buf = String::new();
    print_header();

    fn add(directory: &mut HashMap<String, Vec<String>>, employee: &str, department: &str) {
        directory
            .entry(department.to_string())
            .and_modify(|v| v.push(employee.to_string()))
            .or_insert_with(|| vec![employee.to_string()]);
    }

    fn list_department(directory: &HashMap<String, Vec<String>>, department: &str) {
        if !directory.contains_key(department) {
            println!("> No department {:?} registered.", department);
            return;
        }

        println!("> {}:", department);
        let mut employees: Vec<_> = directory[department].iter().collect();
        employees.sort_unstable();
        for name in employees {
            println!(">   - {}", name);
        }
    }

    fn list_all(directory: &HashMap<String, Vec<String>>) {
        let mut keys: Vec<_> = directory.keys().collect();
        keys.sort_unstable();
        if keys.is_empty() {
            println!("> No departments registered.");
            return;
        }

        for dept_name in keys {
            list_department(directory, dept_name);
        }
    }

    loop {
        buf.clear();
        stdin().read_line(&mut buf).expect("Failed to read line.");

        let m: Vec<_> = buf.trim().split_ascii_whitespace().collect();
        match m[..] {
            ["quit"] => {
                println!("Quitting…");
                return;
            }
            ["add", employee, "to", department] => {
                add(&mut directory, employee, department);
                println!("> added {} to {}", employee, department);
            }
            ["list", "all"] => list_all(&directory),
            ["list", department] => list_department(&directory, department),
            _ => println!("Unrecognised command: {:?}", buf.trim()),
        }
    }
}
