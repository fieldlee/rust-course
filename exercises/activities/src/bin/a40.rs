// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck,
}

#[derive(Debug,PartialEq, PartialOrd,Hash)]
enum Status {
    Available,
    Unavailable,
    Rented,
    Maintenance,
}

#[derive(Debug)]
struct Rental{
    status:Status,
    vehicle:Vehicle,
    vin:String,
}

struct Corporate(Rc<RefCell<Vec<Rental>>>);

struct StoreFront(Rc<RefCell<Vec<Rental>>>);

fn main() {
    let rentals = vec![Rental{
        status: Status::Available,
        vehicle: Vehicle::Car,
        vin:"123".to_owned(),
    },
    Rental{
        status: Status::Unavailable,
        vehicle: Vehicle::Truck,
        vin:"abc".to_owned(),
    }];

    let rentals = Rc::new(RefCell::new(rentals));
    let corporate = Corporate(Rc::clone(&rentals));
    let storefront = StoreFront(Rc::clone(&rentals));

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn update_status(){
        let rentals = vec![Rental{
            status: Status::Available,
            vehicle: Vehicle::Car,
            vin:"123".to_owned(),
        },
        Rental{
            status: Status::Unavailable,
            vehicle: Vehicle::Truck,
            vin:"abc".to_owned(),
        }];
    
        let rentals = Rc::new(RefCell::new(rentals));
        let corporate = Corporate(Rc::clone(&rentals));
        let storefront = StoreFront(Rc::clone(&rentals));

        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                car.status = Status::Rented;
                assert_eq!(car.status,Status::Rented);
            }
        }
        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(truck) = rentals.get_mut(1) {
                truck.status = Status::Rented;
                assert_eq!(truck.status,Status::Rented);
            }
        }
    }
}