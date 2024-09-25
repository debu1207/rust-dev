use std::mem;
// borrow a slice
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // fixed size array
    let xs: [i32; 5] = [3, 1, 4, 2, 6];
    // Initialize to same value
    let ys: [i32; 20] = [0; 20];
    // Use 0 based indexing
    println!("First: {}", xs[0]);
    println!("Second: {}", xs[1]);

    println!("Length: {}", xs.len());

    // arrays are stack allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

}