struct Book {
    url: String,
}

fn get_key_from_id(id: u64) -> u64 {
    id + 1
}

fn fetch_book(id: u64) -> Book {
    let key = get_key_from_id(id);
    Book {
      url: format!("https://<ip-address>/books?id={}&key={}", id, key)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_book() {
        let book = fetch_book(1);
        assert_eq!(book.url, "https://<ip-address>/books?id=1&key=2");
    }
}