pub struct TaxBracket {
    pub limit: f64,
    pub rate: f64,
}

pub enum TaxType {
    Progressive,
}

pub const TAX_BRACKETS: [TaxBracket; 4] = [
    TaxBracket {
        limit: 60_000_000.0,
        rate: 0.05,
    },
    TaxBracket {
        limit: 250_000_000.0,
        rate: 0.15,
    },
    TaxBracket {
        limit: 500_000_000.0,
        rate: 0.25,
    },
    TaxBracket {
        limit: f64::MAX,
        rate: 0.30,
    },
];

pub struct TaxInput {
    pub monthly_income: f64,
    pub tax_type: TaxType,
}
