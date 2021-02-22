use std::borrow::Cow;

fn main() {
	let s = "Hello world!";

	let cow: Cow<str> = Cow::Owned(String::from(s));
	// type mismatch resolving `<str as std::borrow::ToOwned>::Owned == &str`
	//expected struct `std::string::String`, found `&str`

	// ok
	let cow: Cow<str> = Cow::Owned(String::from(s));
}