
fn main() {

    // Clone ladığımızda iki pointer memory adresinde aynı yeri işaret ediyor olmuyor 
    // String in içeriğini kopyalayıp memory de farklı bir yeri allocate edip orayı işaret ediyor
    let original_string = String::from("Hello, world!");
    let cloned_string = original_string.clone(); // Creates a deep copy of the original_string
    
    println!("original_string: {}", original_string);
    println!("cloned_string: {}", cloned_string);

    let original_string = String::from("String");
    let modified_string = modify_string(&original_string);

    println!("original_string: {}", original_string); // "String"
    println!("modified_string: {}", modified_string); // "String modified"
}
fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone(); // Creates a deep copy of the input string
    cloned_string.push_str(" modified");
    cloned_string
}

