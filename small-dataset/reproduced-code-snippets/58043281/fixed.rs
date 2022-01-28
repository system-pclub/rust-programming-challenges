struct Book {
    url: String,
}

fn fetch_book(id: u64) -> Book {
    // do some stuff to get a 'key' variable that allows to access the book
    let key = id + 1;
    Book {
      // Here we turn the formatted String into an &str
      url: format!("https://<ip-address>/books?id={}&key={}", id, key)
    }
}

fn main() {}