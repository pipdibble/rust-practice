trait Printable {
    fn print(&self);
}

struct Book {
    title: String,
}

impl Printable for Book {
    fn print(&self) {
        println!("Book: {}", self.title);
    }
}

struct Document {
    name: String,
}

impl Printable for Document {
    fn print(&self) {
        println!("Document: {}", self.name);
    }
}

fn print_item(item: impl Printable) {
    item.print();
}

fn main() {
    let b = Book { title: String::from("Rust in Action") };
    let d = Document { name: String::from("Curriculum Vitae") };
    print_item(b);
    print_item(d);
}
