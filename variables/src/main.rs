fn main() {
    let mut x = 5;
    println!("x = {}", x);
    x=6;
    println!("x = {}", x);
    const pie:i32 = 3;
    println!("pie = {}", pie);

    let x = 5;
    println!("x = {}", x);
    let x = x+1;
    println!("x = {}", x);
    {
        let x = x*2;
    println!("x = {}", x);
} 

println!("x = {}", x);

}
