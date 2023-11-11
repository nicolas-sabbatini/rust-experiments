trait Command {
    fn execute(&self);
}

struct PrintCommand {
    msg: String,
}
impl Command for PrintCommand {
    fn execute(&self) {
        println!("{}", self.msg);
    }
}

struct SleepCommand {
    s: u64,
}
impl Command for SleepCommand {
    fn execute(&self) {
        std::thread::sleep(std::time::Duration::from_secs(self.s));
    }
}

struct Executor {
    commands: Vec<Box<dyn Command>>,
}
impl Executor {
    fn execute(&mut self) {
        for command in self.commands.drain(0..) {
            command.execute();
        }
    }
}

fn main() {
    let mut executor = Executor {
        commands: vec![
            Box::new(PrintCommand {
                msg: "Hello ".to_string(),
            }),
            Box::new(SleepCommand { s: 3 }),
            Box::new(PrintCommand {
                msg: "World!".to_string(),
            }),
        ],
    };
    executor.execute();
}
