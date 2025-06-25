fn main() {
    let x = 50; 
    {
        let x = 100; 
        println!("Inside block, x = {}", x);
    }

    println!("Outside block, x = {}", x);
}
