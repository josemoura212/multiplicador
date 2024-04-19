fn main() {
    println!("Digite o numero multiplicador!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Valor invalido!");

    let mult = input.trim().parse::<i16>().expect("Digite um numero valido!");

    for i in 0..=10{
        println!("{} x {} = {}", i, mult, i * mult);
    }
}
