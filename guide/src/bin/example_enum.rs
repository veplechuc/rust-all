pub enum State {
    Normal,
    Comment,
    Upper,
    Lower,
}

fn transform(state: State, c: char) -> (Option<char>, State) {
    use self::State::*;

    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, _other) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower),
    }
}

fn main() {
    let mut state = State::Normal;
    let mut processed_string = String::new();
    let input = "This _SHOULD_ be #remove this# in ^lower case^";

    for caracter in input.chars() {
        let (out, new_state) = transform(state, caracter);

        if let Some(output) = out {
            // we use output to reference out
            processed_string.push(output); // add a character not a str
        }
        state = new_state;
    }
    println!("Original: {}", input);
    println!("transformed: {}", processed_string);
}

