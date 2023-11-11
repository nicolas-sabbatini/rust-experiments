fn print(msg: &str) {
    println!("{msg}");
}

fn sleep(s: u64) {
    std::thread::sleep(std::time::Duration::from_secs(s));
}

struct Executor {
    commands: Vec<Box<dyn Fn()>>,
}
impl Executor {
    fn execute(&mut self) {
        for command in self.commands.drain(0..) {
            command();
        }
    }
}

fn main() {
    let mut executor = Executor {
        commands: vec![
            Box::new(|| print("Hello ")),
            Box::new(|| sleep(3)),
            Box::new(|| print("World!")),
        ],
    };
    executor.execute();
}
