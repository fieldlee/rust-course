// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position{
    Maintenance,
    Manager,
    Kitchen,
}
enum Status{
    Active,
    Terminated,
}

struct Employee {
    position:Position,
    status:Status,
}

fn try_access(employee : &Employee) -> Result<(),String>{
    match employee.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }

    match employee.position{
        Position::Manager => Ok(()),
        _ => Err("invaild position".to_owned())
    }
}

fn print_access(employee : &Employee) -> Result<(),String> {
   try_access(employee)
}

fn main(){
    let employee = Employee{
        position: Position::Kitchen,
        status : Status::Active,
    };

    match print_access(&employee){
        Ok(()) => println!("ok"),
        Err(e) => println!("err:{:?}",e),
    }

}