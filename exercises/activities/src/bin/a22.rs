// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {
    let numbers = vec![3,5, 2, 3, 4, 5,40,6];
    let num2: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    println!("num2:{:?}", num2);
    let num3: Vec<_> = numbers.iter().filter(|num| num <= &&3).collect();
    println!("num3:{:?}", num3);
    let findme = numbers.iter().find(|num| num == &&40);
    println!("findme:{:?}", findme);
    let count = numbers.iter().count();
    println!("count:{:?}", count);
    let last = numbers.iter().last();
    println!("last:{:?}", last);

    let min = numbers.iter().min();
    println!("min:{:?}", min);

    let max = numbers.iter().max();
    println!("max:{:?}", max);

    let takes:Vec<_> = numbers.iter().take(3).collect();
    println!("takes:{:?}", takes);


}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test(){
        let expected = clamp(100, 10, 1000);
        assert_eq!(expected,100,"right");
    }
}