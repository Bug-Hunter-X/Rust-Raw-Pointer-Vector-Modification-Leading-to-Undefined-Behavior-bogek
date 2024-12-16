fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Safe and preferred method
    println!( "{:?}", v);

    //Alternative using iter_mut if modifying multiple values
    for i in v.iter_mut(){
        *i = *i * 2
    }
    println!( "{:?}", v);
}