// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let username = "sam";
    match find_user(username) {
        Some(v) => println!("finduser:{:?}", v),
        _ => (),
    };

    let user = find_user(username).map(|id| User {
        user_id: id,
        name: username.to_owned(),
    }).map(|u|{
        println!("user:{:?}",u)
    });

    let some_num = Some(5);
    dbg!(some_num);
    let a_is_some = some_num.is_some();
    dbg!(a_is_some);
    let a_is_none = some_num.is_none();
    dbg!(a_is_none);
    let a_mapped = some_num.map(|num|num+1);
    dbg!(a_mapped);
    let a_filtered = some_num.filter(|num|num==&2);
    dbg!(a_filtered);
    let a_or_else = some_num.or_else(||Some(5));
    dbg!(a_or_else);
    let unwrap = some_num.unwrap_or_else(||0);
    dbg!(unwrap);
}
