// create an 'enum' to classify the day in the week.
enum Day {
    Mon,
    Tue,
    Wed,
    Thurs,
    Fri,
    Sat,
    Sun,
}

enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// function which takes a 'day' enum as argument and returns nothing
fn inspectDay(event: Day) {
    match event {
        Day::Mon => println!("It's Monday"),
        Day::Tue => println!("It's Tuesday"),
        Day::Wed => println!("It's Wednesday"),
        Day::Thurs => println!("It's Thursday"),
        Day::Fri => println!("It's Friday"),
        Day::Sat => println!("It's Saturday"),
        Day::Sun => println!("It's Sunday"),
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let d1 = Day::Wed;
    let d2 = Day::Sat;
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspectDay(d1);
    inspectDay(d2);
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}