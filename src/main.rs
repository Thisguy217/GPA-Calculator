use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter score for math: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let math: f32 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Please enter score for english: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let english: f32 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Please enter score for social study: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let social_study: f32 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Please enter score for science: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let science: f32 = input.trim().parse().unwrap();
    
    let total_classes: f32 = 4.0;

    let total_grades = (&math + &english + &social_study + &science) / &total_classes;

    println!("{}", total_grades);
}

//fn type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}