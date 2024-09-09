fn main() {
    // tuple
    let emp:(&str, u8, u16) = ("Alex", 24, 56000);
    let emp_name = emp.0;
    let emp_age = emp.1;
    let emp_salary = emp.2;

    println!("Employee name: Mr. {}, age: {} year, salary: ${}", emp_name, emp_age, emp_salary);

    // another way:
    let (empl_name, empl_age, empl_sal) = emp;
    println!("Employee name: Mr. {}, age: {} year, salary: ${}", empl_name, empl_age, empl_sal);

}
