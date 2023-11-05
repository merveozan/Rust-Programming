fn main() {
    //Boolean
    let my_first_bool = true;
    let my_second_bool: bool = false;

    // Integers
    // We can say how much memory we need/use
    // 8, 16, 32, 64, 128
    // if we say i8 -> 2^8 = 256 -> 128 negative, 0, 127 positive değere kadar tutabilir 
    let days_of_week:i8= 7;
    let number_of_users:i64 = 1280000;

    // u -> unsigned integer 2^64 değerine kadar positive değer tutabilir 
    let number_of_tokens:u64 = 1000000000;

    //eğer hiçbir şey belirtmezsek otamatik i32 olarak tutar
    let just_a_number = 0;

    //Floating point number
    // 32, 64 (default olarak f64 olarak tutar eğer belirtmezsek)
    let pi:f32 =3.14;

    //Char 
    let my_char ='1';

    //String 
    let message = "Hi, Simon"; // let message : &str  reference immutable

    let my_string = String::from("Hi, Simon");//it can growable, mutable

    //Array
    // We need to say data types and size
    // Once we created it can not change
    //immutable
    let days_of_week:[&str;7] = ["Monday", "Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];

    let first_element = days_of_week[0];
    let final_element = days_of_week[days_of_week.len()-1];

    //Slices 
    let slice = &days_of_week[1..3]; // 1 ve 2. elemanları alır, 3 excluded dahil değil
    let first_element_of_slice = slice[0];

    //Tuples 
    // değişik veri tiplerini tutmaya yarar
    let person = ("Alice",30);

    let name = person.0 ;
    let age = person.1;

    //Unit Type 
    let unit_type = ();// sometimes, we use to make check if this type of unit type or not 

    // variables
    // let num = 5 
    // num=6 this is not allowed because immutability but we can overwrite let num = 6
    let mut num = 5;
    num=6

}