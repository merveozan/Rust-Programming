fn main() {
    let message = "Hello World!"; 

    let x:i32 = 42;
    let pi:f64 = 3.14;
    let is_rust_fun: bool = true;
    let letter_a: char = 'a';

    fn add(x:i32, y:i32) -> i32 {
        x+y  // we can say return x+y;
    }

    let x = -4; // Rust overwrite a izin verir 

    // diğer programlama dillerinden farklı olarak if koşulundakini parentez içine alınmıyor
    if x>=0 { 
        println!("x is not negative");
    } else {
        println!("x is negative");
    }
    
    //In Rust, variables are immutable by default, which means that their values cannot be changed
    let mut i: i32 = 1;
    while i<=5 {
        //Rust dilinde println! , bir format string'i bekler ve bu string içinde süslü parantezler {} yerine geçirilecek argümanlar konulur.
        //Bu, format string'inin sabit bir metin olmasını ve metin içindeki değişkenlerin süslü parantezler kullanılarak gömülmesi gerektiğini ifade eder.
        println!("i'nin değeri: {}", i); // println!("{}", i);
        i+=1;
    }
} 
