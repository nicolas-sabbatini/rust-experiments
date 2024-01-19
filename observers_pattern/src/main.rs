use rand::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq)]
enum Event {
    All,
    Error,
    Save,
}

trait Status {
    fn print_status(&self);
}

trait Observer {
    fn notify(&mut self, message: &Event);
    fn get_id(&self) -> String;
}

struct ObserverObject {
    name: String,
    counter: usize,
    primary_event: Event,
    ticks: usize,
}
impl Observer for ObserverObject {
    fn notify(&mut self, message: &Event) {
        if self.primary_event == *message || self.primary_event == Event::All {
            self.counter += 1;
            println!(
                "\"{}\" has been notified {} times, and is on tick {}",
                self.name, self.counter, self.ticks
            );
        }
    }
    fn get_id(&self) -> String {
        self.name.clone()
    }
}
impl Status for ObserverObject {
    fn print_status(&self) {
        println!(
            "\"{}\" has been notified {} times, and is on tick {}",
            self.name, self.counter, self.ticks
        );
    }
}

trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: &dyn Observer);
    fn notify_observers(&self, message: &Event);
}

struct SubjectObject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    name: String,
}
impl Subject for SubjectObject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        println!(
            "{} has been registered on {}",
            observer.borrow().get_id(),
            self.name
        );
        self.observers.push(observer);
    }
    fn remove_observer(&mut self, observer: &dyn Observer) {
        println!("{} has been removed from {}", observer.get_id(), self.name);
        self.observers
            .retain(|x| x.borrow().get_id() != observer.get_id());
    }
    fn notify_observers(&self, message: &Event) {
        println!("{} is notifying observers", self.name);
        for observer in &self.observers {
            observer.borrow_mut().notify(message);
        }
    }
}
impl Status for SubjectObject {
    fn print_status(&self) {
        println!("{} has {} observers", self.name, self.observers.len());
    }
}

fn main() {
    let mut erroer = SubjectObject {
        observers: vec![],
        name: "ERROER".to_string(),
    };
    let mut saver = SubjectObject {
        observers: vec![],
        name: "SAVER".to_string(),
    };
    let mut aller = SubjectObject {
        observers: vec![],
        name: "ALLER".to_string(),
    };
    let observers = [
        Rc::new(RefCell::new(ObserverObject {
            name: "COUNT SAVE".to_string(),
            counter: 0,
            primary_event: Event::Save,
            ticks: 0,
        })),
        Rc::new(RefCell::new(ObserverObject {
            name: "COUNT ERROR".to_string(),
            counter: 0,
            primary_event: Event::Error,
            ticks: 0,
        })),
        Rc::new(RefCell::new(ObserverObject {
            name: "COUNT ALL".to_string(),
            counter: 0,
            primary_event: Event::All,
            ticks: 0,
        })),
    ];
    for observer in &observers {
        erroer.register_observer(observer.clone());
        saver.register_observer(observer.clone());
        aller.register_observer(observer.clone());
    }
    let mut rng = rand::thread_rng();
    let options = [Event::All, Event::Error, Event::Save];
    for _ in 0..5 {
        for observer in &observers {
            observer.borrow_mut().ticks += 1;
        }
        let event = options.iter().choose(&mut rng).unwrap();
        println!("Sending event {event:?}");
        match event {
            Event::All => {
                aller.notify_observers(event);
            }
            Event::Error => {
                erroer.notify_observers(event);
            }
            Event::Save => {
                saver.notify_observers(event);
            }
        }
    }
    aller.remove_observer(&*observers[2].borrow());
    for _ in 0..5 {
        for observer in &observers {
            observer.borrow_mut().ticks += 1;
        }
        let event = options.iter().choose(&mut rng).unwrap();
        println!("Sending event {event:?}");
        match event {
            Event::All => {
                aller.notify_observers(event);
            }
            Event::Error => {
                erroer.notify_observers(event);
            }
            Event::Save => {
                saver.notify_observers(event);
            }
        }
    }
    aller.print_status();
    erroer.print_status();
    saver.print_status();
    for observer in &observers {
        observer.borrow().print_status();
    }
}
