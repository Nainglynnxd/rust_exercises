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
            let v: Vec<&str> = input.split("").collect();
            let trimmed = &v[1..(v.len() - 2)]; //cut out new line character and two double quotes

            if trimmed.iter().any(|&i| {
                let y: i64 = i.parse().unwrap();
                !(0..=1).contains(&y)
            }) {
                println!("Bruh, I'm out!! Binary Numbers Only!!");
            } else {
                println!("Decimal: {}", binary_to_decimal(trimmed.to_vec()));
            }
        }
    }
}

pub fn decifrac_to_binary(decifrac: &mut f64) -> String {
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

pub fn decimal_to_binary(decimal: &mut i32) -> String {
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

pub fn binary_to_decimal(binary_input: Vec<&str>) -> i64 {
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
