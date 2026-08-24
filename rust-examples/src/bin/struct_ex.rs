/*Book Struct
Create a Book struct with:
title
author
year
Add methods to display its information. */
struct Book {
    title: String,
    author: String,
    year: u32,
}

impl Book {
    fn display_info(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Year: {}", self.year);
    }
}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        year: 2018,
    };

    book.display_info();
}