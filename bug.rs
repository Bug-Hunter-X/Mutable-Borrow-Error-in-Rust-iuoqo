fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y += 1; 
    let z = &mut x; //This line causes the error
    *z +=1;
}