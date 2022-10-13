// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    use std::thread;
    let msg_one = thread::spawn(move||{
        msg_hello()
    });
    let msg_two = thread::spawn(move||{
        msg_thread()
    });
    let msg_three = thread::spawn(move||{
        msg_excited()
    });

    let msg_1 = msg_one.join().expect("one");
    let msg_2 = msg_two.join().expect("two");
    let msg_3 = msg_three.join().expect("three");
    println!("{} {} {}",msg_1,msg_2,msg_3);
}
