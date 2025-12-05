mod display;
mod tax;
mod utils;

use tax::calc::calculate_tax;
use tax::models::{TaxInput, TaxType};

fn main() {
    let salary = utils::input::input_f64("Enter your monthly income: ");

    let tax_input = TaxInput {
        monthly_income: salary,
        tax_type: TaxType::Progressive,
    };

    let (annual_tax, monthly_tax) = calculate_tax(&tax_input);

    display::result::print_result(salary, annual_tax, monthly_tax);
}

//v1

// use core::f64;
// use std::io;

// struct TaxBracket {
//     limit: f64,
//     rate: f64,
// }

// enum TaxType {
//     Progressive,
// }

// const TAX_BRACKETS: [TaxBracket; 4] = [
//     TaxBracket {
//         limit: 60000000.0,
//         rate: 0.5,
//     },
//     TaxBracket {
//         limit: 250000000.0,
//         rate: 0.15,
//     },
//     TaxBracket {
//         limit: 500000000.0,
//         rate: 0.25,
//     },
//     TaxBracket {
//         limit: f64::MAX,
//         rate: 0.30,
//     },
// ];

// struct TaxInput {
//     monthly_income: f64,
//     tax_type: TaxType,
// }

// fn calculate_tax(tax_input: &TaxInput) -> (f64, f64) {
//     match tax_input.tax_type {
//         TaxType::Progressive => calculate_progressive_tax(tax_input.monthly_income),
//     }
// }

// fn calculate_progressive_tax(monthly_income: f64) -> (f64, f64) {
//     let annual_income = monthly_income * 12.0;
//     let mut tax = 0.0;

//     let mut previous_limit = 0.0;

//     for bracket in TAX_BRACKETS {
//         if annual_income > bracket.limit {
//             tax += (bracket.limit - previous_limit) * bracket.rate;
//             previous_limit = bracket.limit;
//         } else {
//             tax += (annual_income - previous_limit) * bracket.rate;
//             break;
//         }
//     }

//     let monthly_tax = tax / 12.0;
//     (tax, monthly_tax)
// }

// fn input_f64(prompt: &str) -> f64 {
//     loop {
//         println!("{}", prompt);
//         let mut buffer = String::new();
//         io::stdin().read_line(&mut buffer).unwrap();

//         match buffer.trim().parse::<f64>() {
//             Ok(v) => return v,
//             Err(_) => println!("Input tidak valid, coba lagi."),
//         }
//     }
// }

// fn main() {
//     let salary = input_f64("Enter your monthly income: ");
//     let tax_input = TaxInput {
//         monthly_income: salary,
//         tax_type: TaxType::Progressive,
//     };

//     let (annual_tax, monthly_tax) = calculate_tax(&tax_input);
//     println!("Monthly income: {:.2}", salary);
//     println!("Annual Tax: {:.2}", annual_tax);
//     println!("Monthly Tax: {:.2}", monthly_tax);
// }
