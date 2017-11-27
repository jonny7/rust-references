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
    let mut s = String::from("Hello");
    change(&mut s);

    // multiple mutable references, uses scope to achieve this
    let mut t = String::from("hello");

    {
        let r1 = &mut t;

    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut t;
}

fn change(some_string: &mut String) {
    some_string.push_str(", World!");
}