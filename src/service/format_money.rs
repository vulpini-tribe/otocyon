use num_format::{Locale, ToFormattedString};
use rusty_money::{iso, Money};

pub fn format_money(monetary_amount: i64, currency: &str) -> String {
    let currency = iso::find(currency).unwrap_or(iso::USD);
    let money = Money::from_major(monetary_amount, currency);
    let currency = money.currency();
    let amount = monetary_amount.to_formatted_string(&Locale::en);

    match currency.symbol_first {
        true => format!("{}\u{2009}{}", currency.symbol, amount),
        false => format!("{}\u{2009}{}", amount, currency.symbol),
    }
}
