use std::io;

fn main() {
    let mut employees: Vec<(String, u8, f32)> = Vec::new();

    for i in 1..=4 {
        println!("Input Data for Employee {}", i);

        let mut name = String::new();
        println!("Name: ");
        io::stdin().read_line(&mut name).expect("Failed to read");
        name = name.trim().to_string();

        let mut age = String::new();
        println!("Age: ");
        io::stdin().read_line(&mut age).expect("Failed to read!");
        let age: u8 = age.trim().parse().expect("Failed to read!");

        let mut salary = String::new();
        println!("Salary: ");
        io::stdin().read_line(&mut salary).expect("Failed to read!");
        let salary: f32 = salary.trim().parse().expect("Failed to read!");

        employees.push((name, age, salary));
    }

    // println!("{:?}", employees);

    for (i, each) in employees.iter().enumerate() {
        println!(
            "Employee {}: Name: {}, Age: {}, Salary: {}",
            i, each.0, each.1, each.2
        )
    }

    let mut max_salary = 0.;
    let mut max_salary_index = 0;
    let mut max_age = 0;
    let mut max_age_index = 0;

    for (index, each) in employees.iter().enumerate() {
        if each.2 > max_salary {
            max_salary = each.2;
            max_salary_index = index;
        }

        if each.1 > max_age {
            max_age = each.1;
            max_age_index = index;
        }
    }

    println!(
        "The employee with the highest salary is: {} with a salary of {}.",
        employees[max_salary_index].0, employees[max_salary_index].2
    );

    println!(
        "The oldest employee is {}, {} years old.",
        employees[max_age_index].0, employees[max_age_index].1
    );

    // Find the employee with the highest salary
    // let highest_salary_employee = employees.iter().max_by_key(|e| e.2).unwrap();

    // Find the oldest employee
    // let oldest_employee = employees.iter().max_by_key(|e| e.1).unwrap();

    // println!(
    //     "The employee with the highest salary is: {} with a salary of {}.",
    //     highest_salary_employee.0, highest_salary_employee.2
    // );

    // println!(
    //     "The oldest employee is {}, {} years old.",
    //     oldest_employee.0, oldest_employee.1
    // );
}
