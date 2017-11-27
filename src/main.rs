/*fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.
*/

fn main() {
    let s = String::from("Hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", World!");
}