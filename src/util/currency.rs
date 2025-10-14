use std::{collections::BTreeMap, sync::LazyLock};

struct CurrencyAmountVisitor;

impl<'de> serde::de::Visitor<'de> for CurrencyAmountVisitor {
    type Value = CurrencyAmount;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a valid currency amount")
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, sqlx::Type)]
#[sqlx(type_name = "CURRENCY_AMOUNT")]
pub struct CurrencyAmount(i64);

impl serde::Serialize for CurrencyAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        <i64 as serde::Serialize>::serialize(&self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for CurrencyAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <i64 as serde::Deserialize<'de>>::deserialize(deserializer).map(Self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub iso_code: CurrencyCode,
    pub symbol: &'static str,
    symbol_is_prefix: bool,
    pub friendly_name: &'static str,
    thousands_separator: char,
    decimal_separator: Option<char>,
}

impl Currency {
    const DECIMAL_PRECISION: u32 = 2;
    const DECIMAL_PART: i64 = 10i64.pow(Self::DECIMAL_PRECISION);

    pub fn get(currency_code: CurrencyCode) -> &'static Self {
        CURRENCIES.get(&currency_code).copied().unwrap()
    }

    pub fn get_all() -> &'static [Currency] {
        &CURRENCY_DEFS
    }

    pub fn parse(&self, value: CurrencyAmount) -> String {
        let mut s = String::new();
        self.parse_into(value, &mut s);
        s
    }

    pub fn parse_into(&self, value: CurrencyAmount, s: &mut String) {
        fn _parse(
            s: &mut String,
            num: i64,
            prev_digit: Option<i64>,
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
            } else if num == 0 && thousands_separator.is_none() {
                for _ in digit_count..Currency::DECIMAL_PRECISION {
                    s.push('0');
                }
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

        let value = value.0;
        if let Some(decimal_separator) = self.decimal_separator {
            let whole_part = value / Self::DECIMAL_PART;
            let frac_part = value % Self::DECIMAL_PART;

            _parse(s, whole_part, None, 0, Some(self.thousands_separator));
            s.push(decimal_separator);
            _parse(s, frac_part, None, 0, None);
        } else {
            _parse(s, value, None, 0, Some(self.thousands_separator));
        }

        if self.symbol_is_prefix {
            s.insert_str(0, self.symbol);
        } else {
            s.push_str(self.symbol);
        }
    }
}

macro_rules! currencies {
    (
        $(
            $currency_code:ident {
                $symbol:literal
                $symbol_is_prefix:literal
                $thousands_separator:literal
                $decimal_separator:expr,
                $friendly_name:literal
            } $(,)?
        )*
    ) => {
        #[allow(clippy::upper_case_acronyms)]
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type,
        )]
        #[sqlx(type_name = "CURRENCY_CODE")]
        pub enum CurrencyCode {
            $( $currency_code, )*
        }

        impl askama::FastWritable for CurrencyCode {
            fn write_into<W: core::fmt::Write + ?Sized>(
                &self,
                dest: &mut W,
                values: &dyn askama::Values,
            ) -> askama::Result<()> {
                match self {
                    $( Self::$currency_code => <str as askama::FastWritable>::write_into(stringify!($currency_code), dest, values), )*
                }
            }
        }

        impl std::fmt::Display for CurrencyCode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <Self as askama::FastWritable>::write_into(self, f, askama::NO_VALUES)?;

                Ok(())
            }
        }

        static CURRENCY_DEFS: [Currency; 18] = [
            $(
                Currency {
                    iso_code: CurrencyCode::$currency_code,
                    symbol: $symbol,
                    symbol_is_prefix: $symbol_is_prefix,
                    friendly_name: $friendly_name,
                    thousands_separator: $thousands_separator,
                    decimal_separator: $decimal_separator
                },
            )*
        ];
    }
}

currencies! {
    USD { "$"  true  ',' Some('.'), "United states dollar" },
    EUR { "€"  true  ',' Some('.'), "Euro" },
    JPY { "¥"  true  ',' None,      "Japanese yen" },
    GBP { "£"  true  ',' Some('.'), "Sterling" },
    CNY { "¥"  true  ',' None,      "Renminbi" },
    AUD { "$"  true  ',' Some('.'), "Australian dollar" },
    CAD { "$"  true  ',' Some('.'), "Canadian dollar" },
    CHF { "Fr" false '.' Some('.'), "Swiss franc" },
    HKD { "$"  true  ',' Some('.'), "Hong Kong dollar" },
    SGD { "$"  true  ',' Some('.'), "Singapore dollar" },
    SEK { "kr" false '.' Some('.'), "Swedish krona" },
    KRW { "₩"  true  ',' None,      "South Korean won" },
    NOK { "kr" false ',' None,      "Norwegian krone" },
    NZD { "$"  true  ',' Some('.'), "New Zealand dollar" },
    INR { "₹"  true  ',' None,      "Indian rupee" },
    MXN { "$"  true  ',' Some('.'), "Mexican peso" },
    TWD { "$"  true  ',' Some('.'), "New Taiwan dollar" },
    ZAR { "R"  true  ',' None,      "South African rand" },
}

static CURRENCIES: LazyLock<BTreeMap<CurrencyCode, &'static Currency>> = LazyLock::new(|| {
    BTreeMap::from_iter(
        CURRENCY_DEFS
            .iter()
            .map(|currency| (currency.iso_code, currency)),
    )
});
