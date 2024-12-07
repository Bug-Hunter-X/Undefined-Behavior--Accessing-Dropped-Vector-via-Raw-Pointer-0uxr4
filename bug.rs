fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modifies the first element
    }
    println!("v: {:?}", v); // v is modified

    // Using the pointer after the vector is dropped causes undefined behavior.
    drop(v); // v is dropped here. 
    unsafe {
        *ptr = 5; //Undefined Behavior
    }
    println!("v: {:?}", v); // This line is unreachable.
}