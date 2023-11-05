
fn main() {
    let sum = add(3,5);
    println!("The sum is {} ", sum);

    let day_of_week = "Sunday";

    if day_of_week == "Sunday"{
        println!("The race day");
    } else if day_of_week =="Saturday" {
        println!("Qualifying day");
    }else {
        println!("Patiently wait the race day");
    }

    //while 
    let mut counter =0 ;

    while counter <=5 {
        println!("Counter value is {}",counter);
        counter+=1;
    }

    //for loop 
    for number in 1..=5 {
        println!("Number is {}",number);
    }

    let numbers:[i32;5] = [10,20,30,40,50];
    for number in numbers {
        println!("Number is {}",number);
    }

    //loop 
    //(we can use for infinite, if we set as true we can say you work until this is false if we can do this in while loop)
    // it similar to do while in c 
    counter = 0;
    loop {
        println!("Counter while is {}", counter);
        counter+=1;

        if counter==6{
            break
        }
    }

    //match
    // it looks like switch case 
    let num = 2;
    match num {
        1 => { // if num is equal to 1
            println!("The number is one ");
            println!("This is the first match arm");
        }
        2 => println!("The number is one "),
        3 => println!("The number is one "),
        _ => println!("The number is something else "),// eğer num yukarıdakilerden hiçbiri değilse
        
    }

    let result = match num { // hangisine uyuyorsa string veri tipinde tutucak variable a değeri atayacak
        1 => "The number is one!",
        2 => "The number is two!",
        3 => "The number is three!",
        _ => "The number is something else!",
    };
   
    println!("The result is {}", result);// variable ın değerini ekrana bastırıyoruz    

}
// parametre olarak 2 int tipinde alıyor ve return value sı da integer tipinde
fn add(x:i32,y:i32) ->i32 {
    let result = x+y ;
    return result;
}

// it does not give any parameters and return any value
fn no_param() {
    println!("This is just works");
}
