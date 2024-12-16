fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safer method using indexing
    println!("{:?}", v);

    let mut v2 = vec![1,2,3];
    //Safer method using borrowing 
    { //this ensures that the borrow is valid until the end of the block 
        let first_element = &mut v2[0]; 
        *first_element = 4;
    }
    println!("{:?}", v2);
} 