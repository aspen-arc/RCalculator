use std::io;

fn main() {
    println!("Enter number 1:");
    
    let mut num1 = String::new();

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line.");

    let _int_num1: i64 = num1
        .trim()
        .parse()
        .unwrap();

    println!("Enter number 2:");
    
    let mut num2 = String::new();

    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line.");

    let _int_num2: i64 = num2
        .trim()
        .parse()
        .unwrap();

    println!("Method: \n 1.) Addidtion \n 2.) Subtraction \n 3.) Multiplication \n 4.) Divison");

    let mut method = String::new();

    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line.");

    let _int_method: i64 = method
        .trim()
        .parse()
        .unwrap();

    if _int_method == 1 {
        let answer: i64 = _int_num1 + _int_num2;

        println!("\n {} + {} = {}",_int_num1, _int_num2, answer)
    } else if _int_method == 2 {
        let answer: i64 = _int_num1 - _int_num2;

        println!("\n {} - {} = {}",_int_num1, _int_num2, answer)
    } else if _int_method == 3 {
        let answer: i64 = _int_num1 * _int_num2;

        println!("\n {} * {} = {}",_int_num1, _int_num2, answer)
    } else if _int_method == 4 {
        let answer: i64 = _int_num1 / _int_num2;

        println!("\n {} / {} = {}",_int_num1, _int_num2, answer)
    } else {
        println!("\n Invalid option.")
    }
}