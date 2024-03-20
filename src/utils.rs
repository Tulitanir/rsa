use rand::Rng;

pub fn get_keys() -> (i64, i64, i64) {
    let primes = get_primes(1000000);

    let len = primes.len();
    let index = len * 1 / 2;
    let mut rng = rand::thread_rng();

    let prime_index1 = rng.gen_range(index..=len-1);
    let prime_index2 = {
        let mut tmp = rng.gen_range(index..=len-1);

        loop {
            if tmp != prime_index1 {
                break;
            }

            tmp = rng.gen_range(index..=len-1);
        }

        tmp
    };

    let prime1 = primes[prime_index1];
    let prime2 = primes[prime_index2];
    let n = prime1 * prime2;
    let euler_func = (prime1 - 1) * (prime2 - 1);

    //открытая экспонента должна быть взаимнопростой с выходом функции Эйлера
    //закртыая экспонента - число, которое при умножении на открытую экспоненту по модулю выхода функции Эйлера даст 1
    let (open_exponent, closed_exponent) = {
        let index = prime_index1.max(prime_index2);

        let mut res = primes[1];
        let mut x = 1;

        for i in (1..index).rev() {
            let (gcd, x1, _) = gcde(primes[i], euler_func);
            if gcd == 1 {
                res = primes[i];
                x = x1;
                break;
            }
        }

        if x < 0 {
            x += euler_func as i64;
        }

        (res, x)
    };

    (n, open_exponent, closed_exponent)
}

pub fn encrypt(message: &str, n: i64, open_exponent: i64) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::with_capacity(message.len());

    for byte in message.bytes() {
        res.push(exponentiation(byte as i128, open_exponent, n));
    }

    res
}

pub fn decrypt(cypher: &Vec<i64>, n: i64, closed_exponent: i64) -> String {
    let mut res: Vec<u8> = Vec::with_capacity(cypher.len());

    for i in 0..cypher.len() {
        res.push(exponentiation(cypher[i] as i128, closed_exponent, n) as u8);
    }

    String::from_utf8(res).unwrap()
}

fn get_primes(maximum: i64) -> Vec<i64> {
    let mut primes = vec![2];

    for candidate in 3..maximum {
        let square_root = (candidate as f64).sqrt() as i64 + 1;
        let is_prime = primes
            .iter()
            .take_while(|p| p <= &&square_root)
            .all(|p| candidate % p != 0);
        if is_prime {
            primes.push(candidate);
        }
    }

    primes
}

fn gcde(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }

    let x: i64;
    let y: i64;

    let (gcd, x1, y1) = gcde(b % a, a);
    x = y1 - (b / a) as i64 * x1;
    y = x1;

    (gcd, x, y)
}

fn exponentiation(mut base: i128, mut exponent: i64, div: i64) -> i64 {
    let mut result = 1;

    while exponent > 0 {
        if exponent & 0x01 == 0x01 {
            result = (result * base) % div as i128;
        }

        base = base * base % div as i128;
        exponent = exponent >> 1;
    }

    result as i64
}