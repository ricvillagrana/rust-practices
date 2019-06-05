struct Company {
    name: String,
    employees: Vec<Person>,
}

impl Company {
    fn total_payed_per_month(&self) -> f32 { self.employees.iter().map( |e| e.salary ).sum() }
}

struct Person {
    name: String,
    position: String,
    salary: f32,
}

impl Person {
    fn daily_salary(&self) -> f32 { self.salary/30.0 }
    //fn hourly_salary(&self) -> f32 { self.salary/160.0 }
}

fn calculate_salary(employee: &Person, days: i32) -> Option<f32> {
    if employee.salary <= 0.0 {
        return None;
    }
    Some(employee.daily_salary() * days as f32)
}

fn main() {
    let mut employees = Vec::new();

    employees.push(Person {
        name: "Ricardo Villagrana".to_string(),
        position: "Software Engineer".to_string(),
        salary: 100.00,
    });

    employees.push(Person {
        name: "Jorge Precich".to_string(),
        position: "Sr Software Engineer".to_string(),
        salary: 200.00,
    });

    employees.push(Person {
        name: "Pedro Terriquez".to_string(),
        position: "Trainee".to_string(),
        salary: 0.0,
    });


    let tango = Company {
        name: "CompanySource LLC.".to_string(),
        employees
    };

    println!("{} is actually paying ${} monthly to their employees.", tango.name, tango.total_payed_per_month());

    for employee in &tango.employees {
        let days = 7;
        let salary = calculate_salary(&employee, days);

        if salary == None {
            println!("\n{} [{}] is not being paid. :(", employee.name, employee.position);
        } else {
            println!("\nThe salary of {} day(s) of {} [{}] is ${:?}", days, employee.name, employee.position, salary.unwrap());
            println!("{} earns ${:?}/hour.", employee.name, employee.salary/160.0)
        }
    }
}
