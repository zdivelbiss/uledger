#![allow(dead_code, clippy::upper_case_acronyms)]

use std::num::NonZeroU32;

#[derive(Debug)]
pub enum Kind {
    Currency(CurrencyKind),
    Equity(EquityKind),
}

macro_rules! commodity_kind {
    (
        $(#[$outer:meta])*
        $vis:vis enum $Def:ident {
            $(
                $(#[$inner:meta])*
                $Variant:ident { $ThsSep:expr, $Dec:expr }
            ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis enum $Def {
            $(
                $(#[$inner])*
                $Variant
            ),*
        }

    impl From<$Def> for Commodity {
        fn from(value: $Def) -> Commodity {
            match value {
                $(
                    $Def::$Variant => const {
                        match $Dec {
                            Some((decimal_separator, decimal_precision)) => {
                                if let Some(decimal_precision) = NonZeroU32::new(decimal_precision) {
                                    Commodity::new(stringify!($Variant), $ThsSep, Option::Some((decimal_separator, decimal_precision)))
                                } else {
                                    panic!("zero decimal precision not allowed")
                                }
                            }

                            None => Commodity::new(stringify!($Variant), $ThsSep, None)
                        }

                    },
                )*
            }
        }
    }

    };
}

commodity_kind! {
    #[derive(Debug)]
    pub enum CurrencyKind {
        USD { ',', Some(('.', 2)) },
        EUR { ',', Some(('.', 2)) },
        JPY { ',', None },
        GBP { ',', Some(('.', 2)) },
        CNY { ',', None },
        AUD { ',', Some(('.', 2)) },
        CAD { ',', Some(('.', 2)) },
        CHF { '.', Some(('.', 2)) },
        HKD { ',', Some(('.', 2)) },
        SGD { ',', Some(('.', 2)) },
        SEK { '.', Some(('.', 2)) },
        KRW { ',', None },
        NOK { ',', None },
        NZD { ',', Some(('.', 2)) },
        INR { ',', None },
        MXN { ',', Some(('.', 2)) },
        TWD { ',', Some(('.', 2)) },
        ZAR { ',', None },
    }
}

commodity_kind! {
    #[derive(Debug)]
    pub enum EquityKind {
        AAPL { ',', Some(('.', 3)) },
    }
}

#[derive(Debug, Clone)]
pub struct Commodity {
    symbol: &'static str,
    thousands_separator: char,
    decimals: Option<(char, NonZeroU32)>,
}

impl Commodity {
    const MAX_DECIMAL_PRECISION: u32 = 9;
    const DECIMAL_PART: u128 = 10u128.pow(Self::MAX_DECIMAL_PRECISION);

    pub const fn new(
        symbol: &'static str,
        thousands_separator: char,
        decimals: Option<(char, NonZeroU32)>,
    ) -> Self {
        Self {
            symbol,
            thousands_separator,
            decimals,
        }
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
                if num > 0 && (digit_count % 3) == 0 {
                    if let Some(thousands_separator) = thousands_separator {
                        s.push(thousands_separator);
                    }
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
                    _ => unreachable!("last_digit out of range"),
                }
            }
        }

        if let Some((decimal_separator, decimal_precision)) = self.decimals {
            debug_assert!(decimal_precision.get() <= Self::MAX_DECIMAL_PRECISION);

            let whole_part = value / Self::DECIMAL_PART;
            let frac_part = (value % Self::DECIMAL_PART)
                / 10u128.pow(Self::MAX_DECIMAL_PRECISION - decimal_precision.get());

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
