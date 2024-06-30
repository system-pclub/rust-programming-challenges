trait EventsHandler {
    fn on_event(&mut self);
}
struct Engine {
    data: String,
}
struct Request {
    data: String,
}

struct RequestHandler<'a> {
    r: &'a mut Request,
}
impl<'a> EventsHandler for RequestHandler<'a> {
    fn on_event(&mut self) {
        self.r.data.push_str("request handler on event\n");
    }
}

impl Engine {
    pub fn exec(&mut self, script: &str, mut handler: Box<dyn EventsHandler>) {
        handler.on_event();
        self.data.push_str(script);
    }
}

fn handle_request(script: &str, r: &mut Request) -> String {
    let r_h: RequestHandler = RequestHandler { r };
    let handler = Box::new(r_h);
    let mut engine = Engine {
        data: "".to_string(),
    };
    engine.exec(script, handler);
    format!("engine data: {}\n handler data: {}", engine.data, r.data)
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_request() {
        let mut r = Request {
            data: "request data\n".to_string(),
        };
        let result = handle_request("script", &mut r);
        assert_eq!(
            result,
            "engine data: script\n handler data: request data\nrequest handler on event\n"
        );
    }
}