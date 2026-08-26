/*Library System
Model:
books
members
borrowing
returning books */
struct Book {
    title: String,
    author: String,
    borrowed: bool,
}
#[derive(Clone)]
struct Member {
    name: String,
}

struct Library {
    books: Vec<Book>,
    members: Vec<Member>,
}

impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new(),
            members: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn add_member(&mut self, member: Member) {
        self.members.push(member);
    }

    fn borrow_book(&mut self, title: &str, member: &Member) {
        for book in &mut self.books {
            if book.title == title {
                if !book.borrowed {
                    book.borrowed = true;
                    println!("Book '{}' by {} has been borrowed by {}.", title, book.author, member.name);
                    return;
                } else {
                    println!("Book '{}' is already borrowed.", title);
                    return;
                }
            }
        }
        println!("Book '{}' not found.", title);

    }

    fn return_book(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                if book.borrowed {
                    book.borrowed = false;
                    println!("Book '{}' has been returned.", title);
                    return;
                } else {
                    println!("Book '{}' was not borrowed.", title);
                    return;
                }
            }
        }
        println!("Book '{}' not found.", title);
    }
}

fn main() {
    let mut library = Library::new();

    let book1 = Book {
        title: "The Great Gatsby".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
        borrowed: false,
    };

    let book2 = Book {
        title: "To Kill a Mockingbird".to_string(),
        author: "Harper Lee".to_string(),
        borrowed: false,
    };

    let member1 = Member {
        name: "John Doe".to_string(),
    };

    let member2 = Member {
        name: "Jane Smith".to_string(),
    };

    library.add_book(book1);
    library.add_book(book2);
    library.add_member(member1.clone());
    library.add_member(member2.clone());

    library.borrow_book("The Great Gatsby", &member1);
    library.borrow_book("The Great Gatsby", &member2);
    library.return_book("The Great Gatsby");
    library.return_book("To Kill a Mockingbird");
}
