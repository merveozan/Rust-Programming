fn main() {
    let book = Book {
        title: String::from("The Way of Zen"),
        author: String::from("Allan Wats"),
        publication_year: 1957,
    };
    println!("The book {} is written by {} in {} ",
     book.title,book.author,book.publication_year);

     let mut book = Book {
        title: String::from("The Way of Zen"),
        author: String::from("Allan Wats"),
        publication_year: 1957,
    };

    book.publication_year = 1989;

    println!("The book {} is written by {} in {} ",
     book.title,book.author,book.publication_year);

    let book_data = get_book_data(book);

    for data in book_data {
        println!("{data}");
    }
    
    let my_book = create_book("The Path of Zen".to_string(), "Simon".to_string(), 2023);

    println!("My book is {:?}",my_book);

    let tuple_book = Tuple_Book("Some Book".to_string(),"Simon".to_string(),2023);

    let title = tuple_book.0;
    let author = tuple_book.1;
    let publication_year = tuple_book.2;

    let unit_book = Unit_Book();

    let my_rectangle = Rectangle {
        width:10.0,
        height : 5.0,
    };
    let area = my_rectangle.area();
    println!("The area of the rectangle {}", area);
    

}
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

struct Tuple_Book(String,String,u32);

struct Unit_Book();


// String tipinde 3 elemanlı liste
fn get_book_data(book : Book) -> [String; 3] {
        let title = book.title;
        let author = book.author;
        let publication_year = book.publication_year;
        let data : [String;3] = [title,author,publication_year.to_string()];
        data
}

//Constructor
fn create_book(title:String,author:String,publication_year:u32) -> Book {
    let book = Book {
        title:title,
        author:author,
        publication_year:publication_year,

    };
    book

}

struct Rectangle {
    width: f64,
    height : f64,
}
impl Rectangle{
    fn area(&self) -> f64 {
        self.width * self.height
    }
}