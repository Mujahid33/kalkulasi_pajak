use super::models::{TAX_BRACKETS, TaxInput, TaxType};

pub fn calculate_tax(tax_input: &TaxInput) -> (f64, f64) {
    match tax_input.tax_type {
        TaxType::Progressive => calculate_progressive_tax(tax_input.monthly_income),
    }
}

fn calculate_progressive_tax(monthly_income: f64) -> (f64, f64) {
    let annual_income = monthly_income * 12.0;
    let mut tax = 0.0;

    let mut previous_limit = 0.0;

    for bracket in TAX_BRACKETS {
        if annual_income > bracket.limit {
            tax += (bracket.limit - previous_limit) * bracket.rate;
            previous_limit = bracket.limit;
        } else {
            tax += (annual_income - previous_limit) * bracket.rate;
            break;
        }
    }

    let monthly_tax = tax / 12.0;
    (tax, monthly_tax)
}
