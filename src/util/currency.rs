use std::{collections::BTreeMap, sync::LazyLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub iso_code: CurrencyCode,
    pub symbol: &'static str,
    pub friendly_name: &'static str,
    thousands_separator: char,
    decimal_separator: Option<char>,
}

impl Currency {
    const MAX_DECIMAL_PRECISION: u32 = 9;
    const DECIMAL_PART: u128 = 10u128.pow(Self::MAX_DECIMAL_PRECISION);

    pub fn get(currency_code: CurrencyCode) -> Option<&'static Self> {
        CURRENCIES.get(&currency_code).copied()
    }

    pub fn get_all() -> &'static [Currency] {
        &CURRENCY_DEFS
    }

    pub fn get_serialized(currency_code: CurrencyCode) -> Option<&'static str> {
        INDIVIDUAL_CURRENCIES_SERIALIZED
            .get(&currency_code)
            .map(String::as_str)
    }

    pub fn get_all_serialized() -> &'static str {
        &ALL_CURRENCIES_SERIALIZED
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

macro_rules! currencies {
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
        #[allow(non_camel_case_types)]
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type,
        )]
        #[sqlx(type_name = "CURRENCY_CODE")]
        pub enum CurrencyCode {
            $( $currency_code, )*
        }

        impl std::fmt::Display for CurrencyCode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( Self::$currency_code => f.write_str(stringify!($currency_code)), )*
                }
            }
        }

        static CURRENCY_DEFS: [Currency; 18] = [
            $(
                Currency {
                    iso_code: CurrencyCode::$currency_code,
                    symbol: $symbol,
                    friendly_name: $friendly_name,
                    thousands_separator: $thousands_separator,
                    decimal_separator: $decimal_separator
                },
            )*
        ];
    }
}

currencies! {
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

static CURRENCIES: LazyLock<BTreeMap<CurrencyCode, &'static Currency>> = LazyLock::new(|| {
    BTreeMap::from_iter(
        CURRENCY_DEFS
            .iter()
            .map(|currency| (currency.iso_code, currency)),
    )
});

static ALL_CURRENCIES_SERIALIZED: LazyLock<String> = LazyLock::new(|| {
    serde_json::to_string(&CURRENCY_DEFS).expect("failed to serialize currencies")
});

static INDIVIDUAL_CURRENCIES_SERIALIZED: LazyLock<BTreeMap<CurrencyCode, String>> =
    LazyLock::new(|| {
        BTreeMap::from_iter(CURRENCY_DEFS.iter().map(|currency| {
            (
                currency.iso_code,
                serde_json::to_string(currency).expect("failed to serialize currency"),
            )
        }))
    });
