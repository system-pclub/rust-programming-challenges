struct FontLoader(String);
struct Font<'a>(&'a str);

impl FontLoader {
    fn load(&self) -> Font {
        Font(&self.0)
    }
}

struct Window;

struct Phi<'window> {
    window: &'window Window,
    loader: FontLoader,
    font: Option<Font<'window>>,
}

impl<'window> Phi<'window> {
    fn do_the_thing(&mut self) {
        let font = self.loader.load();
        self.font = Some(font);
    }
}

fn main() {}
