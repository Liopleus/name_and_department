use std::collections::HashMap;
use std::io;

fn main() {
    println!("This program allows you to add new entry to an employee database containing the name and department.");
    let mut dept_count: u16 = 0;
    let mut dept_list: HashMap<u16, String> = HashMap::new();
    let mut employee_data: HashMap<String, u16> = HashMap::new();

    loop {
        let option_add_employee = 1;
        let option_create_dept = 2;
        let option_list_by_dept = 3;
        let option_list_all = 4;
        let option_exit = 9;

        println!("Choose what you want to do:");
        println!("({}) Add an employee to the database", option_add_employee);
        println!("({}) Add a new department to the database",option_create_dept);
        println!("({}) List the employees in a department",option_list_by_dept);
        println!("({}) List all employees in the company", option_list_all);
        println!("({}) Exit the program", option_exit);

        let mut mode = String::new();
        io::stdin().read_line(&mut mode).expect("Invalid input!");

        match mode.trim().parse() {
            Ok(num) => {
                let choice: i32 = num;
                if choice == option_add_employee {
                    add_employee(&mut employee_data, &dept_list, &dept_count);
                    continue;
                } else if choice == option_create_dept {
                    create_dept(&mut dept_count, &mut dept_list);
                    continue;
                } else if choice == option_list_by_dept {
                    list_by_department(&employee_data, &dept_list, &dept_count);
                    continue;
                } else if choice == option_list_all {
                    list_all(&employee_data, &dept_list, &dept_count);
                    continue;
                } else if choice == option_exit {
                    break;
                } else {
                    println!("Please enter a number corresponding to an option!");
                    continue;
                }
            }

            _ => {
                println!("Please enter a number corresponding to an option!");
                continue;
            }
        }
    }
}

fn add_employee(
    database: &mut HashMap<String, u16>,
    dept_list: &HashMap<u16, String>,
    dept_count: &u16,
) {
    if *dept_count == 0 {
        no_department();
        println!("Please add some departments to the database before you add employees.");
        return;
    }

    println!("What's the name of this employee?");
    let mut name = String::new();

    loop {
        name.clear();
        io::stdin().read_line(&mut name).expect("Invalid input!");
        if name.trim().chars().count() > 0 {
            break;
        } else {
            println!("Please enter a valid name!");
            continue;
        }
    }

    println!("What department does this employee belong to?");
    let id = dept_selection(dept_list);
    database.insert(String::from(name.trim()), id);
    println!(
        "An employee named {} has been add to the {} department.",
        name.trim(),
        dept_list.get(&id).unwrap()
    );
}

fn create_dept(
    count: &mut u16, 
    list: &mut HashMap<u16, String>
) {
    println!("Please enter the name of the department:");
    let mut department = String::new();

    loop {
        department.clear();
        io::stdin()
            .read_line(&mut department)
            .expect("Invalid input!");

        if department.trim().chars().count() > 0 {
            break;
        } else {
            println!("Please enter a valid department name!");
            continue;
        }
    }

    *count += 1;
    list.insert(*count, String::from(department.trim()));
    println!(
        "The {} department has been added to the database.",
        department.trim()
    );
}

fn list_by_department(
    database: &HashMap<String, u16>,
    dept_list: &HashMap<u16, String>,
    dept_count: &u16,
) {
    if *dept_count == 0 {
        no_department();
        return;
    }

    // Match the number specified by the user with the department and push the names in that department into the output vector
    println!("What department's employee list do you want to view?");
    println!("Please choose a department with the numbers below:");
    let id = dept_selection(dept_list);
    let mut output: Vec<&String> = Vec::new();
    for (name, dept_id) in database {
        if id == *dept_id {
            output.push(name)
        }
    }

    // Sort the output vector and print the elements inside
    if output.len() == 0 {
        println!("There's current no employee in this department!");
    } else {
        output.sort();
        for name in output {
            println!("{}", name)
        }
    }
}

fn list_all(
    database: &HashMap<String, u16>, 
    dept_list: &HashMap<u16, String>, 
    dept_count: &u16
) {
    if *dept_count == 0 {
        no_department();
        return;
    }

    // Create a list of departments sorted alphabetically, then convert to corresponding IDs
    let mut sorted_ids: Vec<u16> = Vec::new();
    {
        let mut dept_sorted: Vec<&String> = Vec::new();
        for (_id, name) in dept_list {
            dept_sorted.push(name);
        }
        dept_sorted.sort();

        for dept_name in dept_sorted {
            for (id, name) in dept_list {
                if name == dept_name {
                    sorted_ids.push(*id)
                }
            }
        }
    }

    for id in sorted_ids {
        let mut output: Vec<&String> = Vec::new();
        for (name, dept_id) in database {
            if *dept_id == id {
                output.push(name);
            }
        }
        output.sort();

        let dept_name = dept_list.get(&id).unwrap();
        for name in output {
            println!("{}, {}", name, dept_name)
        }
    }
}

fn no_department() {
    println!("There's currently no department in the database!")
}

// This function takes a user input, validate if it's a number corresponding to a department in dept_list and returns that number
fn dept_selection(dept_list: &HashMap<u16, String>) -> u16 {
    for (id, name) in dept_list {
        println!("({}) {}", id, name)
    }

    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid input!");

        match choice.trim().parse() {
            Ok(num) => {
                let target_id: u16 = num;
                match dept_list.get(&target_id) {
                    Some(_) => return target_id,

                    None => {
                        println!("Please enter a number corresponding to a department!");
                        continue;
                    }
                }
            }

            Err(_) => {
                println!("Please enter a number corresponding to a department!");
                continue;
            }
        }
    }
}