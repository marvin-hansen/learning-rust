use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Unknown
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Coin::Penny => write!(f, "Penny"),
            Coin::Nickel => write!(f, "Nickel"),
            Coin::Dime => write!(f, "Dime"),
            Coin::Quarter(_) => write!(f, "Quarter"),
        }
    }
}

pub(crate) fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_in_cents(){
        let penny = Coin::Penny;
        assert_eq!(value_in_cents(penny), 1);
        let nickel = Coin::Nickel;
        assert_eq!(value_in_cents(nickel), 5);
        let dime = Coin::Dime;
        assert_eq!(value_in_cents(dime), 10);
        let quarter = Coin::Quarter(UsState::Unknown);
        assert_eq!(value_in_cents(quarter), 25);
    }
}