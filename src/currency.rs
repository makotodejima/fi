pub enum Currency {
    EUR,
    JPY,
    USD,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Currency::EUR => "EUR",
            Currency::JPY => "JPY",
            Currency::USD => "USD",
        }
    }
    pub fn the_others(&self) -> [Self; 2] {
        match *self {
            Currency::EUR => [Currency::JPY, Currency::USD],
            Currency::JPY => [Currency::EUR, Currency::USD],
            Currency::USD => [Currency::EUR, Currency::JPY],
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "eu" | "eur" | "euro" | "EUR" => Currency::EUR,
            "jp" | "jpy" | "yen" | "JPY" => Currency::JPY,
            "us" | "usd" | "dollar" | "USD" => Currency::USD,
            _ => panic!("Invalid currency symbol given"),
        }
    }
}
