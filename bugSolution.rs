fn main() {
    let mut v = vec![1, 2, 3];
    let mut v2 = v.clone();
    { //create a new scope.
        let ptr = v2.as_mut_ptr();
        unsafe {
            *ptr = 4; // Modifies the first element
        }
    }    
    println!("v2: {:?}", v2); // v is modified
    
    println!("v: {:?}", v);
}