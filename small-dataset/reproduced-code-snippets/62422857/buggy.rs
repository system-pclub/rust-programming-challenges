struct Layer{
	name: String,  // Human readable name
	id: String,    // UUID in the future
	order: u8,     // int for sorting
	width: u8,     // Number of nodes
	input: [&'Self],  // References to other Layers that feed input into this
}

fn main() {
	
}