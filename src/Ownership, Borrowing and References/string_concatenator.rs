
fn main() {
    // Step 4: In the main function, create two String variables.
    let string1 = String::from("Hello, ");
    let string2 = String::from("World!");

    // Step 5: Call the concatenate_strings function with string slices of the variables.
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Step 6: Print the result to the console.
    println!("{}", concatenated_string);
}


// Step 1 & 2: Write the concatenate_strings function signature and implement the function.
fn concatenate_strings(s1: &str, s2: &str) -> String {
    // Step 2: Inside the concatenate_strings function, create a new String called result.
    let mut result = String::new();

    // Use the push_str() method to append the contents of both input string slices.
    result.push_str(s1);
    result.push_str(s2);

    // Step 3: Return the result string from the function.
    result
}
