fn main() {
    let mut x = 5;
    { // creating a block
        let y = &mut x; 
        *y += 1; 
    }
    let z = &mut x; 
    *z +=1; 
}