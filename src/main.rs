use std::io::{stdin, stdout, Write};

fn main() {
    let mut decimal = String::new();
    print!("Decimal: ");

    let _ = stdout().flush();

    stdin().read_line(&mut decimal).expect("decimal not found!");

    let mut x: f64 = decimal.trim().parse().expect("can't find float");

    if x % 1f64 == 0f64 {
        println!("Binary: {}", decimal_to_binary(&mut (x as i32)));
    } else {
        println!("Binary: {}", decifrac_to_binary(&mut x));
    }
}

fn decifrac_to_binary(decifrac: &mut f64) -> String {
    let mut binary = String::new();
    let mut decimal = *decifrac as i32;
    let mut frac = ((*decifrac - *decifrac as i64 as f64) * 100.0).round() / 100.0;

    for _ in 0..=3 {
        frac = frac * 2f64;
        if frac >= 1f64 {
            binary.push_str(&1.to_string());
            frac = ((frac - frac as i64 as f64) * 100.0).round() / 100.0;
        } else {
            binary.push_str(&(frac as i64).to_string());
        }
    }

    format!("{}.{}", decimal_to_binary(&mut decimal), binary)
}

fn decimal_to_binary(decimal: &mut i32) -> String {
    let mut binary = String::new();

    loop {
        if *decimal / 2 == 0 {
            binary.push_str(&(*decimal % 2).to_string());
            return binary.chars().rev().collect::<String>();
        } else {
            binary.push_str(&(*decimal % 2).to_string());
            *decimal = *decimal / 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{decifrac_to_binary, decimal_to_binary};
    #[test]
    fn decimalonly_works() {
        let result = decimal_to_binary(&mut 8);
        assert_eq!(result, "1000".to_owned());
    }

    #[test]
    fn fraction_works() {
        let result = decifrac_to_binary(&mut 3.14);
        assert_eq!(result, "11.0010".to_owned());
    }
}
