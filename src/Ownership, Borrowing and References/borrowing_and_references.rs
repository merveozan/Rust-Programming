
fn main() {
    let my_string = String::from("Hello World");

    //We can get value of my_string (Hello World) using my_ref but 
    //we can not modify the value through my_ref
    let my_ref = &my_string;

    println!("My reference is : {}", my_ref);

    let my_string = String::from("Hello World");

    //print_string(&my_string);
    //println!("I still got my string {}", my_string);
    let mut my_string = String::from("Hello ");
    change_string(&mut my_string);
   // println!("{}", my_string);
   let first_immutable_reference = &my_string;
   let second_immutable_reference = &my_string;

   //println!("First immutable reference value: {}, Second immutable reference value: {}",first_immutable_reference,second_immutable_reference);

   //rust is so intelligence it say that we are not going to use immutable after we changed to mutable there wouldn't be any conflict
    let first_mutable_reference = &mut my_string;
    println!("First mutable reference value {}",first_mutable_reference);

    // In rust, after we create this mutable reference, the first immutable reference is no longer valid
    // Because of this gives an error, 
   // println!("First immutable reference value: {}", first_immutable_reference);


   
   let second_mutable_reference = &mut my_string;
   println!("Second mutable reference value {}",second_mutable_reference);

    //It gives an error with the same ideology after we got our second mutable reference we can not use our first mutable reference
    // once the conflict start rust puts a stop here 
    
   //println!("First mutable reference value {}",first_mutable_reference);


// NOTE : In summary,You can have either one mutable reference or any number of immutable reference to a variable at a time 
    let new_string = String::from("new string");
    let new_string_ref = return_reference(&new_string);
   // println!("new string : {}", new_string);

   //after we say this ownership moves to newer_string
   //two pointers wouldn't point the same memory on the heap
   // we deallocateed new_string

    let newer_string = new_string;
  //  println!("new string reference {}", new_string_ref);

}
fn print_string(s: &String) {
    println!("{}", s);
}

fn change_string(s: &mut String) {
    s.push_str(" world");
}
fn return_reference(some_string: &String) -> &String {
    some_string
} // some_string goes out of scope, but reference_to_s still points to the memory location