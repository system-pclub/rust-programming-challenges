struct CowRow {
    pub id : i32,
}
impl CowRow {
    fn get_id(&self) -> i32       { self.id }
}

fn agg<F>(col: F) -> Box<FnMut(&CowRow) -> i32>
    where F: Fn(&CowRow) -> i32 {
    let mut res = 0;
    Box::new(move |r| { res += col(&r); return res })
}

fn main() {
    let mut cow = CowRow { id: 0 };
    let a = agg(CowRow::get_id);
    a(&cow);
}