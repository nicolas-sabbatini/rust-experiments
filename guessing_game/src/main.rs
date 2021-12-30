use io_handle::*;

mod io_handle;

enum State {
    Playing,
    Endgame,
    Restart,
    Quit,
}

fn main() {
    flush_text("Lets play a game 🎮! (y/n) > ");
    // Check if the player wants to play
    match read_line().as_ref() {
        "y" => start_game(),
        "n" => flush_line("Okay then 😞."),
        _ => flush_line("I didn't understand."),
    }
    flush_line("Goodbye!");
}

fn start_game() {
    let mut variables = restart_game();
    loop {
        match variables.0 {
            State::Playing => variables = play_game(variables.1, variables.2),
            State::Endgame => variables.0 = end_game(),
            State::Restart => variables = restart_game(),
            State::Quit => break,
        }
    }
}

fn play_game(min: f32, max: f32) -> (State, f32, f32) {
    flush_text("Let me think ");
    timed_flush_text("......", 2.0);
    let middle = (min + max) / 2.0;
    flush_line(&format!(" Is your number {}", middle as usize));
    flush_text("My number is (l)ower/(h)igher/(e)qual/(q)uit > ");
    let res = read_line();
    if &res != "e" && min as usize == max as usize {
        flush_line("You are a layer!! 😡💢");
        flush_line("I don't want to play with you anymore!! 😡💢");
        return (State::Quit, min, max);
    }
    match res.as_ref() {
        "l" => (State::Playing, min, middle),
        "h" => (State::Playing, middle + 0.5, max),
        "e" => (State::Endgame, min, max),
        "q" => (State::Quit, min, max),
        _ => {
            flush_line("I didn't understand.");
            (State::Playing, min, max)
        }
    }
}

fn end_game() -> State {
    flush_line("Yey!! I win!! 🥳🎉");
    flush_text("Do you want to play again? (y/n) > ");
    match read_line().as_ref() {
        "y" => State::Restart,
        "n" => State::Quit,
        _ => {
            flush_line("I didn't understand.");
            State::Endgame
        }
    }
}

fn restart_game() -> (State, f32, f32) {
    flush_line("Think a number between 0 and 100");
    timed_flush_text("..........", 3.0);
    flush_line(" Ready?");
    flush_line("Now I am going to guess that number! 🧙✨");
    (State::Playing, 0.0, 100.0)
}
