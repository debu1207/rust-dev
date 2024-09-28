// Globals (outside all scopes)

static LANGUAGE: &str = "Python";
const LIMIT: u16 = 4096;

fn is_big(n: u16) -> bool {
    n > LIMIT
}

fn main() {
    let n = 4312;
    // access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The limit is {}", LIMIT);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});

    // LIMIT = 3333; Can't change a 'const'
}