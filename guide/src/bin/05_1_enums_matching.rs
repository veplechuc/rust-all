#![allow(dead_code)]

enum IpAdrr {
    V4(u8, u8, u8, u8),
    V6,
}

enum Message {
    Quit,                    // stores no data
    Move { x: i32, y: i32 }, // stores a anonymous struct
    Write(String),           // single string
    ChangeColor(i128),       // store a 128 integer
}

enum Something {
    Number(i32),
    #[allow(unused)] // whit this line, the compiler wont complain if not used
    Name(String),
    Unknown,
}

enum Colour {
    Red,
    #[allow(unused)] // whit this line, the compiler wont complain if not used
    White,
}

enum AnotherColor {
    Red,
    Blue,
    Rgb(u8, u8, u8),                    // this is s tuple member
    Invented { cyan: u8, magenta: u8 }, //this is s struct like member
}

// ENUM OPTION

// The Option enum is used to represent a variety of situations, such as:

// The result of a function call that may or may not return a value.
// A value that may be missing from a database or other data structure.
// A value that may be null in a language like C or Java.

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Example of enums with struct
#[derive(Debug)]
struct Custom {
    age: usize,
    name: String,
}
#[derive(Debug)]
enum Item {
    Number(usize),
    Mycustom(Custom),
}

fn app_item(v: &mut Vec<Item>) {
    v.push(Item::Number(8));
    v.push(Item::Mycustom(Custom {
        age: 22,
        name: "hola".to_string(),
    }));
}

enum State {
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

    for c in input.chars() {
        let (out, new_state) = transform(state, c);

        if let Some(output) = out {
            processed_string.push(output);
        }
        state = new_state;
    }
    println!("Original: {}", input);
    println!("transformed: {}", processed_string);

    println!("//////////////////////////////");
    let mut vec: Vec<Item> = vec![];
    app_item(&mut vec);
    println!("{:?}", vec.get(1));
    println!("{:?}", vec[0]);
    println!("//////////////////////////////");

    let s: Something = Something::Name("some name".to_string()); // we need to specify the value
    let s: Something = Something::Number(5); // we need to specify the value
    match s {
        Something::Name(n) => println!("el nombre es ->{}", n), // extract the value to a new variable
        Something::Number(n) => println!("el numero es ->{}", n), // extract the value to a new variable
        _ => println!("Nothing..."),
    }
    let c: Colour = Colour::Red;

    match c {
        Colour::Red => println!("el color es Rojo"),
        _ => println!("no taken into account"),
    }

    let another: AnotherColor = AnotherColor::Invented {
        cyan: 10,
        magenta: 20,
    };

    match another {
        //     AnotherColor::Red => println!("Rojo"),
        //     AnotherColor::RGB(, , )
        AnotherColor::Invented {
            cyan: _,
            magenta: 122,
        } => println!("invented"),

        _ => println!("no taken into account"),
    }

    // MORE EXAMPLES
    // consecutuves values..
    enum Vals {
        Var1, // 0
        Var2 = 10000,
        Var3, // 10001
    }

    println!("var1: {}", Vals::Var1 as u32);
    println!("var2: {}", Vals::Var2 as u32);
    println!("var3: {}", Vals::Var3 as u32);

    // Create an Enum for days of week
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    // Use enum to store todays day
    let today: Day = Day::Monday;

    // Perform different actions based on day
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Hung out day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday | Day::Sunday => println!("Weekend!!!"),
    }

    // Check if today is a weekend
    //println!("Is today the weekend {}", today.is_weekend());

    let greetings = [
        "Hello",
        "Hola",
        "Bonjour",
        "Ciao",
        "こんにちは",
        "안녕하세요",
        "Cześć",
        "Olá",
        "Здравствуйте",
        "Chào bạn",
        "您好",
        "Hallo",
        "Hej",
        "Ahoj",
        "سلام",
        "สวัสดี",
    ];

    for (num, greeting) in greetings.iter().enumerate() {
        print!("{} : ", greeting);
        match num {
            0 => println!("This code is editable and runnable!"),
            1 => println!("¡Este código es editable y ejecutable!"),
            2 => println!("Ce code est modifiable et exécutable !"),
            3 => println!("Questo codice è modificabile ed eseguibile!"),
            4 => println!("このコードは編集して実行出来ます！"),
            5 => println!("여기에서 코드를 수정하고 실행할 수 있습니다!"),
            6 => println!("Ten kod można edytować oraz uruchomić!"),
            7 => println!("Este código é editável e executável!"),
            8 => println!("Этот код можно отредактировать и запустить!"),
            9 => println!("Bạn có thể edit và run code trực tiếp!"),
            10 => println!("这段代码是可以编辑并且能够运行的！"),
            11 => println!("Dieser Code kann bearbeitet und ausgeführt werden!"),
            12 => println!("Den här koden kan redigeras och köras!"),
            13 => println!("Tento kód můžete upravit a spustit"),
            14 => println!("این کد قابلیت ویرایش و اجرا دارد!"),
            15 => println!("โค้ดนี้สามารถแก้ไขได้และรันได้"),
            _ => {}
        }
    }
}
