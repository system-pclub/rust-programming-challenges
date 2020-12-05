use std::collections::HashMap;

struct Test {}

#[derive(Hash, Eq, PartialEq, Debug)]
struct InputCellID {}

#[derive(Hash, Eq, PartialEq, Debug)]
struct ComputeCellID {}

impl Test {
	pub fn broken_set_value<'a>(&'a mut self, id: InputCellID) {
		let (original_value_map, end_value_map) = self.update_listeners(&id);
		self.fire_callbacks(original_value_map, end_value_map);
	}

	fn update_listeners<'a>(
		&'a mut self,
		_id: &InputCellID,
	) -> (
		HashMap<&'a ComputeCellID, Option<i32>>,
		HashMap<&'a ComputeCellID, Option<i32>>,
	) {
		let original_values = HashMap::new();
		let end_values = HashMap::new();
		(original_values, end_values)
	}

	fn fire_callbacks<'a>(
		&self,
		original_value_map: HashMap<&'a ComputeCellID, Option<i32>>,
		end_value_map: HashMap<&'a ComputeCellID, Option<i32>>,
	) {
		for (cell_id, original_value) in original_value_map.iter() {
			let end_value = end_value_map.get(cell_id).unwrap();
			if end_value != original_value && end_value.is_some() {
				// Fire callbacks for current cell_id.
			}
		}
	}
}


fn main() {
	
}
