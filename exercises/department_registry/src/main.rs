use std::{collections, io};

fn main() {
    let mut registry = collections::HashMap::<String, String>::new();

    println!("Enter an action. Type \"h\" for a list of actions.");
    loop {
        let mut user_input = String::new();
        if let Err(_) = io::stdin().read_line(&mut user_input) {
            println!("Failed to read line. Try again.");
            continue;
        }

        let args: Vec<&str> = user_input.split_whitespace().collect();

        if is_help_command(&args) {
            help();
        } else if is_quit_command(&args) {
            return;
        } else if is_add_command(&args) {
            let name = args[1].to_lowercase();
            let department = args[3].to_lowercase();
            registry.insert(name, department);
        } else if is_get_person(&args) {
            let name = args[1].to_lowercase();
            match registry.get(&name) {
                Some(department) => {
                    println!("{} works in {}", name, department);
                }
                None => {
                    println!("Employee not found");
                }
            }
        } else if is_list_department_command(&args) {
            let department = args[1].to_lowercase();

            let mut employees: Vec<String> = Vec::new();
            for employee in registry.keys() {
                if let Some(current_department) = registry.get(employee)
                    && *current_department == department
                {
                    employees.push(employee.to_string());
                }
            }
            if employees.len() == 0 {
                println!("There are no employees in that department");
            } else {
                println!("The employees of that department are:");
                for employee in employees {
                    println!("- {}", employee);
                }
            }
        } else if is_list_all_command(&args) {
            let mut employees_by_department = collections::HashMap::<String, Vec<&str>>::new();

            for employee in registry.keys() {
                if let Some(dep) = registry.get(employee) {
                    employees_by_department
                        .entry(dep.clone())
                        .or_insert(Vec::new())
                        .push(employee);
                }
            }

            if employees_by_department.keys().len() == 0 {
                println!("No employee found");
            }

            for (department, employees) in employees_by_department.iter() {
                println!("{department}");
                for employee in employees {
                    println!("- {employee}");
                }
            }
        } else {
            println!("Unknown command.")
        }
    }
}

fn help() {
    println!("Command:");
    println!("Add an individial to a department     add [name] to [department]");
    println!("List all people from a department     list [department]");
    println!("List all people from all departments  list");
}

fn is_quit_command(args: &Vec<&str>) -> bool {
    args.len() == 1 && (args[0].to_lowercase() == "quit" || args[0].to_lowercase() == "q")
}

fn is_add_command(args: &Vec<&str>) -> bool {
    args.len() == 4 && args[0].to_lowercase() == "add" && args[2].to_lowercase() == "to"
}

fn is_get_person(args: &Vec<&str>) -> bool {
    args.len() == 2 && args[0].to_lowercase() == "get"
}

fn is_list_all_command(args: &Vec<&str>) -> bool {
    args.len() == 1 && args[0].to_lowercase() == "list"
}

fn is_list_department_command(args: &Vec<&str>) -> bool {
    args.len() == 2 && args[0].to_lowercase() == "list"
}

fn is_help_command(args: &Vec<&str>) -> bool {
    args.len() == 1 && (args[0].to_lowercase() == "help" || args[0].to_lowercase() == "h")
}
