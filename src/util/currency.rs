use std::{collections::BTreeMap, sync::LazyLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct Commodity {
    currency_code: &'static str,
    symbol: &'static str,
    friendly_name: &'static str,
    thousands_separator: char,
    decimal_separator: Option<char>,
}

impl Commodity {
    const MAX_DECIMAL_PRECISION: u32 = 9;
    const DECIMAL_PART: u128 = 10u128.pow(Self::MAX_DECIMAL_PRECISION);

    pub fn get(currency_code: &str) -> Option<&'static Self> {
        COMMODITIES.get(currency_code).copied()
    }

    pub fn get_all() -> &'static [Commodity] {
        &COMMODITY_DEFS
    }

    pub fn get_serialized(currency_code: &str) -> Option<&'static str> {
        INDIVIDUAL_COMMODITIES_SERIALIZED
            .get(currency_code)
            .map(String::as_str)
    }

    pub fn get_all_serialized() -> &'static str {
        &ALL_COMMODITIES_SERIALIZED
    }

    pub fn parse(&self, value: u128) -> String {
        let mut s = String::new();
        self.parse_into(value, &mut s);
        s
    }

    pub fn parse_into(&self, value: u128, output: &mut String) {
        fn _parse(
            s: &mut String,
            num: u128,
            prev_digit: Option<u128>,
            digit_count: u32,
            thousands_separator: Option<char>,
        ) {
            if num > 0 {
                _parse(
                    s,
                    num / 10,
                    Some(num % 10),
                    digit_count + 1,
                    thousands_separator,
                );
            }

            if let Some(prev_digit) = prev_digit {
                if num > 0
                    && digit_count.is_multiple_of(3)
                    && let Some(thousands_separator) = thousands_separator
                {
                    s.push(thousands_separator);
                }

                match prev_digit {
                    0 => s.push('0'),
                    1 => s.push('1'),
                    2 => s.push('2'),
                    3 => s.push('3'),
                    4 => s.push('4'),
                    5 => s.push('5'),
                    6 => s.push('6'),
                    7 => s.push('7'),
                    8 => s.push('8'),
                    9 => s.push('9'),
                    _ => unreachable!(),
                }
            }
        }

        if let Some(decimal_separator) = self.decimal_separator {
            let whole_part = value / Self::DECIMAL_PART;
            let frac_part =
                (value % Self::DECIMAL_PART) / 10u128.pow(Self::MAX_DECIMAL_PRECISION - 2);

            _parse(output, whole_part, None, 0, Some(self.thousands_separator));
            output.push(decimal_separator);
            _parse(output, frac_part, None, 0, None);
        } else {
            _parse(output, value, None, 0, Some(self.thousands_separator));
        }

        output.push(' ');
        output.push_str(self.symbol);
    }
}

macro_rules! commodities {
    (
        $(
            $currency_code:ident {
                $symbol:literal
                $thousands_separator:literal
                $decimal_separator:expr,
                $friendly_name:literal
            } $(,)?
        )*
    ) => {
        static COMMODITY_DEFS: [Commodity; 18] = [
            $(
                Commodity {
                    currency_code: stringify!($currency_code),
                    symbol: $symbol,
                    friendly_name: $friendly_name,
                    thousands_separator: $thousands_separator,
                    decimal_separator: $decimal_separator
                },
            )*
        ];
    }
}

commodities! {
    USD { "$"   ',' Some('.'), "United states dollar" },
    EUR { "€"   ',' Some('.'), "Euro" },
    JPY { "¥"   ',' None,      "Japanese yen" },
    GBP { "£"   ',' Some('.'), "Sterling" },
    CNY { "¥"   ',' None,      "Renminbi" },
    AUD { "$"   ',' Some('.'), "Australian dollar" },
    CAD { "$"   ',' Some('.'), "Canadian dollar" },
    CHF { "Fr"  '.' Some('.'), "Swiss franc" },
    HKD { "$"   ',' Some('.'), "Hong Kong dollar" },
    SGD { "$"   ',' Some('.'), "Singapore dollar" },
    SEK { "kr"  '.' Some('.'), "Swedish krona" },
    KRW { "₩"   ',' None,      "South Korean won" },
    NOK { "kr"  ',' None,      "Norwegian krone" },
    NZD { "$"   ',' Some('.'), "New Zealand dollar" },
    INR { "₹"   ',' None,      "Indian rupee" },
    MXN { "$"   ',' Some('.'), "Mexican peso" },
    TWD { "$"   ',' Some('.'), "New Taiwan dollar" },
    ZAR { "R"   ',' None,      "South African rand" },
}

static COMMODITIES: LazyLock<BTreeMap<&'static str, &'static Commodity>> = LazyLock::new(|| {
    BTreeMap::from_iter(
        COMMODITY_DEFS
            .iter()
            .map(|commodity| (commodity.currency_code, commodity)),
    )
});

static ALL_COMMODITIES_SERIALIZED: LazyLock<String> = LazyLock::new(|| {
    serde_json::to_string(&COMMODITY_DEFS).expect("failed to serialize commodities")
});

static INDIVIDUAL_COMMODITIES_SERIALIZED: LazyLock<BTreeMap<&'static str, String>> =
    LazyLock::new(|| {
        BTreeMap::from_iter(COMMODITY_DEFS.iter().map(|commodity| {
            (
                commodity.currency_code,
                serde_json::to_string(commodity).expect("failed to serialize commodity"),
            )
        }))
    });
