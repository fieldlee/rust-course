// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::io;
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub struct Bill {
    name:String,
    amount:f32,
}

pub struct Bills {
    inner : HashMap<String,Bill>,
}
impl Bills {
    fn new()->Self{
        Self { inner: HashMap::new() }
    }
    fn add(&mut self,name:String,bill:Bill){
        self.inner.insert(name,bill);
    }
    fn remove(&mut self,name:String){
        self.inner.remove(name.as_str());
    }
    fn get_all(&self)->Vec<&Bill> {
        self.inner.values().collect()
    }
    fn update(&mut self,name:String,bill:Bill){

        self.inner.remove(name.as_str());
        self.inner.insert(name, bill);
    }
}

fn get_input()->Option<String>{
    let mut buffer = String::new();
    let user_input = io::stdin().read_line(&mut buffer);

    match  user_input{
        Ok(_) => {
            let input = buffer.trim().to_lowercase();
            if input.is_empty() {
                return None
            }else{
                return Some(input)
            }
        },
        Err(_) => return None
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    EditBill,
}

impl MainMenu {
    fn from_str(input : &str) ->Option<MainMenu>{
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            "3" => Some(MainMenu::RemoveBill),
            "4" => Some(MainMenu::EditBill),
            _ => None,
        }
    }
    fn show(){

    }
}

mod menu{
    use crate::get_input;
    use crate::Bills;
    use crate::Bill;
    pub fn view_bills(bills :&Bills){
        for bill in bills.inner.iter(){
            println!("name:{:?},amount:{:?}",bill.0,bill.1);
        }
    }

    pub fn remove_bill(bills :&mut Bills){
        let name = match get_input(){
            Some(v) => v,
            None => return,
        };

        bills.remove(name);
    }

    pub fn add_bill(bills : &mut Bills){

        let name = match get_input(){
            Some(v) => v,
            None => return,
        };
        let amount = match get_input() {
            Some(v) => {
                match v.parse(){
                    Ok(f) =>  f,
                    Err(_) => return,
                }
            },
            None => return,
        };
        let namekey = name.clone();
        let bill = Bill{name,amount};
        bills.add(namekey,bill);
        println!("bill added !")
    }

    pub fn update_bill(bills : &mut Bills){
        remove_bill(bills);
        add_bill(bills);
    }
}


fn main() {
    let mut bills = Bills::new();

    loop {
        let input=get_input().expect("input value error");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::EditBill) => menu::update_bill(&mut bills),
            None => return,
        }
    }
}
