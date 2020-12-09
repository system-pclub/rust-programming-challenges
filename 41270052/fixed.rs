struct FontLoader(String);
struct Font<'a>(&'a str);

impl FontLoader {
    fn load(&self) -> Font {
        Font(&self.0)
    }
}

struct Window;

struct Phi<'a> {
    window: &'a Window,
    loader: &'a FontLoader,
    font: Option<Font<'a>>,
}

impl<'a> Phi<'a> {
    fn do_the_thing(&mut self) {
        let font = self.loader.load();
        self.font = Some(font);
    }
}

fn main() { }
