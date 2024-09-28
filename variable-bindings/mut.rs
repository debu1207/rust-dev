fn main() {
    let _immutable_binding = 1; // for mutabilty add mut
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    //_immutable_binding += 1;
}