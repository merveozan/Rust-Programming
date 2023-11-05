
fn main() {
    
    // Strings are stored in heap strings are dynamic value that can grow and shrink
    // size ını runtime da öğreniyoruz daha öncesinden bilinmiyor 
    let s1 = String::from("hello");

    // ownership transfer
    let s2 = s1 ;


    // it gives an error because 
    //if two different pointers pointing the same loacation
    //to avoid this Rust eliminate s1 
    //println!(" value of s1: {}",s1);

    let x = 5 ; // stored on the stack because we know size 
    let y  = String::from("patika");
    let z=y ;
    println!("value of x : {}, value of z : {} ",x,z);
}
