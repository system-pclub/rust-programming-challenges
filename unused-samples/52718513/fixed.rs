pub struct Shell {
    child: std::process::Child,
}

impl Shell {
    pub fn init() -> Shell {
        let mut cmd = std::process::Command::new("ls");
        let process = cmd.spawn();
        let new = Shell {
            child: process.unwrap(),
        };
        new
    }

    pub fn f1(&mut self) {
        //do something with self
    }

    pub fn f2(mut self) {
        {
            let stdin = self.child.stdin.as_mut().unwrap();
        }
        let output = self.child.wait_with_output();
    }
}

fn main() {
    let mut shell = Shell::init();
    shell.f1();
    shell.f2();
}