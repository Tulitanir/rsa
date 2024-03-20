use rsa::utils::*;

fn main() {
    let (n, open_exponent, closed_exponent) = get_keys();
    println!("N: {}, открытая экспонента: {}, закрытая экспонента: {}", n, open_exponent, closed_exponent);

    let text = "Hello, world!";

    let cypher = encrypt(text, n, open_exponent);
    println!("{:?}", cypher);

    let decoded_text = decrypt(&cypher, n, closed_exponent);
    println!("{}", decoded_text);
}
