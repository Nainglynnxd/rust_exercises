use std::io::{stdin, stdout, Write};

enum FlagOption {
    DecimalToBinary,
    BinaryToDecimal,
}

pub fn run(flag: &str) {
    use FlagOption::*;
    let mut input = String::new();
    let option: FlagOption;
    match flag {
        "db" => {
            option = DecimalToBinary;
            print!("Decimal: ");
        }
        "bd" => {
            option = BinaryToDecimal;
            print!("Binary: ");
        }
        _ => option = DecimalToBinary,
    }
    let _ = stdout().flush();
    stdin().read_line(&mut input).unwrap();

    match option {
        DecimalToBinary => {
            let mut x: f64 = input.trim().parse().expect("can't find float");
            if x % 1f64 == 0f64 {
                println!("Binary: {}", decimal_to_binary(&mut (x as i32)));
            } else {
                println!("Binary: {}", decifrac_to_binary(&mut x));
            }
        }
        BinaryToDecimal => {
            if input.contains('.') {
                let input = input.split('.').collect::<Vec<&str>>();
                println!("{}", bifrac_to_decifrac(input[0], input[1].trim()));
            } else {
                let mut input = cast_char_to_string(&input);
                input.remove(input.len() - 1);
                println!("Decimal: {}", binary_to_decimal(input));
            }
        }
    }
}

fn decifrac_to_binary(decifrac: &mut f64) -> String {
    let mut binary = String::new();
    let mut decimal = *decifrac as i32;
    let mut frac = ((*decifrac - *decifrac as i64 as f64) * 100.0).round() / 100.0;

    for _ in 0..=3 {
        frac *= 2f64;
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
            *decimal /= 2;
        }
    }
}

fn binary_to_decimal(binary_input: Vec<String>) -> i64 {
    let mut decimal = 0;

    for (index, value) in binary_input.iter().enumerate() {
        let mut factor = 1;
        if *value == "1" {
            for _ in 0..(binary_input.len() - index) - 1 {
                factor *= 2;
            }
            decimal += factor;
        }
    }
    decimal
}

fn check_validity_input(input: &str) {
    let input = cast_char_to_string(input);

    for k in input.iter() {
        let num = k.parse::<i64>().unwrap();
        if num > 1 {
            eprintln!("0 and 1 only bruh!! I'm out!!");
            std::process::exit(1);
        }
    }
}

fn bifrac_to_decifrac(binary: &str, fraction: &str) -> String {
    check_validity_input(binary);
    check_validity_input(fraction);

    let mut sum_of_fraction = 0.0;
    let fraction = cast_char_to_string(fraction);
    let binary = cast_char_to_string(binary);
    let mut factor = 1f64;
    for value in fraction.iter() {
        factor *= 2.0;
        let k: f64 = value.parse().unwrap();
        sum_of_fraction += k / factor;
    }

    format!(
        "Decimal: {}",
        binary_to_decimal(binary) as f64 + sum_of_fraction
    )
}

fn cast_char_to_string(input: &str) -> Vec<String> {
    //cast each char into String and store inside Vec
    input
        .chars()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
}
