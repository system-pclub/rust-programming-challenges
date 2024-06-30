// Doesn't really matter what this struct contains, it just needs an owning method
struct SideStruct;
impl SideStruct {
    fn something_side<A: TraitA>(&self, mut aval: A) -> String {
        format!("something sideways :)\n{}", aval.something_a(42))
    }
}

trait TraitA {
    fn something_a(&mut self, data: u32) -> String; // this would be the meat of my logic
}

// Note that this struct has an explicit lifetime
struct MainStruct<'a> {
    refr: &'a mut u32
}

// Note that I implement for a mutable reference to MainStruct
impl<'a> TraitA for &'a mut MainStruct<'a> {
    fn something_a(&mut self, data: u32) -> String {
        // Completely arbitrary, can safely ignore this function body
        *self.refr += data;
        // println!("We're finally doing something: {}", self.refr);
        format!("We're finally doing something: {}", self.refr)
    }
}

// Implementing for MainStruct itself
impl<'a> MainStruct<'a> {
    // Note, I can't change the signature for this function because it implements a trait
    fn something_indirect(&mut self, ss: &SideStruct) -> String {
        // here is where the error occurs!
        ss.something_side(self)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let mut base_val: u32 = 42;
        let ss = SideStruct {};
        let mut main_val = MainStruct { refr: &mut base_val };
        assert_eq!(main_val.something_indirect(&ss), "something sideways :)\nWe're finally doing something: 84");
    }
}