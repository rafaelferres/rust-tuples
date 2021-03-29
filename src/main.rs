fn main() {
    let tupla = (5, "teste", 1.2, (1, 2, 3));
    let (a, b, c, d) = tupla;

    println!("O valor de a é {}", a);
    println!("O valor de b é {}", b);
    println!("O valor de c é {}", c);
    println!("O valor de d é {:?}", d);
}
