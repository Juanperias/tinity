use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum TypeError {
    #[error("The sum could not be made because the types do not match.")]
    MismatchedTypes,

    #[error("there was an overflow in the sum")]
    Overflow,

    #[error("there was an error trying to cast {0}")]
    CannotCast(Type),

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    I8(i8),
    U8(u8),

    I16(i16),
    U16(u16),

    I32(i32),
    U32(u32),

    I64(i64),
    U64(u64),

    I128(i128),
    U128(u128),

    // Registers or Variables, starts with %
    Value(String),
}


impl Type {
    pub fn try_add(&mut self, val: Type) -> Result<(), TypeError> {
        match (self, val) {
            (Type::I8(a), Type::I8(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::U8(a), Type::U8(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::I16(a), Type::I16(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::U16(a), Type::U16(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::I32(a), Type::I32(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::U32(a), Type::U32(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::I64(a), Type::I64(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::U64(a), Type::U64(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::I128(a), Type::I128(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            (Type::U128(a), Type::U128(b)) => {
                *a = a.checked_add(b).ok_or(TypeError::Overflow)?;
            }
            _ => return Err(TypeError::MismatchedTypes),
        }
        Ok(())
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[macro_export]
macro_rules! type_from_string {
    ($t:expr, $v:expr) => {{
        fn is_integer(input: &str) -> bool {
            if input.is_empty() {
                return false;
            }

            let mut chars = input.chars();

            if let Some(first_char) = chars.next() {
                if !first_char.is_digit(10) && first_char != '-' {
                    return false;
                }
            } else {
                return false;
            }

            chars.all(|c| c.is_digit(10))
        }
        if !is_integer($v) {
            $crate::parser::types::Type::from($v)
        } else {
            match $t {
                "i8" => {
                    $crate::parser::types::Type::from($v.parse::<i8>().expect("Failed to parse"))
                }
                "u8" => {
                    $crate::parser::types::Type::from($v.parse::<u8>().expect("Failed to parse"))
                }
                "i16" => {
                    $crate::parser::types::Type::from($v.parse::<i16>().expect("Failed to parse"))
                }
                "u16" => {
                    $crate::parser::types::Type::from($v.parse::<u16>().expect("Failed to parse"))
                }
                "i32" => {
                    $crate::parser::types::Type::from($v.parse::<i32>().expect("Failed to parse"))
                }
                "u32" => {
                    $crate::parser::types::Type::from($v.parse::<u32>().expect("Failed to parse"))
                }
                "i64" => {
                    $crate::parser::types::Type::from($v.parse::<i64>().expect("Failed to parse"))
                }
                "u64" => {
                    $crate::parser::types::Type::from($v.parse::<u64>().expect("Failed to parse"))
                }
                "i128" => {
                    $crate::parser::types::Type::from($v.parse::<i128>().expect("Failed to parse"))
                }
                "u128" => {
                    $crate::parser::types::Type::from($v.parse::<u128>().expect("Failed to parse"))
                }
                _ => panic!("Unsupported type"),
            }
        }
    }};
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Type::from(value.to_string())
    }
}


macro_rules! to_type {
    ($($t:ty, $variant:ident),*) => {
        $(
            impl From<$t> for Type {
                fn from(value: $t) -> Self {
                    Type::$variant(value)
                }
            }
        )*
    };
}

macro_rules! into_val {
    ($($t:ty, $variant:ident),*) => {
        $(
            impl TryInto<$t> for Type {
                type Error = TypeError;

                fn try_into(self) -> Result<$t, Self::Error> {
                    match self {
                        Self::$variant(v) => {
                            Ok(v)
                        },
                        _ => {
                            Err(TypeError::CannotCast(self))
                        }
                    }
                }
            }
        )*
    }
}

to_type!(
    i8, I8, u8, U8, i16, I16, u16, U16, i32, I32, u32, U32, i64, I64, u64, U64, i128, I128, u128,
    U128, String, Value
);

into_val!(
    i8, I8, u8, U8, i16, I16, u16, U16, i32, I32, u32, U32, i64, I64, u64, U64, i128, I128, u128,
    U128, String, Value
);
