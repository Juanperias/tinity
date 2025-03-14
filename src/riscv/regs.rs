use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Reg {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
    Mstatus,
    Mepc,
    Mtvec,
    Mcause,
}

impl FromStr for Reg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "zero" => Ok(Reg::Zero),
            "ra" => Ok(Reg::Ra),
            "sp" => Ok(Reg::Sp),
            "gp" => Ok(Reg::Gp),
            "tp" => Ok(Reg::Tp),
            "t0" => Ok(Reg::T0),
            "t1" => Ok(Reg::T1),
            "t2" => Ok(Reg::T2),
            "s0" => Ok(Reg::S0),
            "s1" => Ok(Reg::S1),
            "a0" => Ok(Reg::A0),
            "a1" => Ok(Reg::A1),
            "a2" => Ok(Reg::A2),
            "a3" => Ok(Reg::A3),
            "a4" => Ok(Reg::A4),
            "a5" => Ok(Reg::A5),
            "a6" => Ok(Reg::A6),
            "a7" => Ok(Reg::A7),
            "s2" => Ok(Reg::S2),
            "s3" => Ok(Reg::S3),
            "s4" => Ok(Reg::S4),
            "s5" => Ok(Reg::S5),
            "s6" => Ok(Reg::S6),
            "s7" => Ok(Reg::S7),
            "s8" => Ok(Reg::S8),
            "s9" => Ok(Reg::S9),
            "s10" => Ok(Reg::S10),
            "s11" => Ok(Reg::S11),
            "t3" => Ok(Reg::T3),
            "t4" => Ok(Reg::T4),
            "t5" => Ok(Reg::T5),
            "t6" => Ok(Reg::T6),
            "mstatus" => Ok(Reg::Mstatus),
            "mepc" => Ok(Reg::Mepc),
            "mtvec" => Ok(Reg::Mtvec),
            "mcause" => Ok(Reg::Mcause),
            _ => Err(format!("Unknown register: {}", s)),
        }
    }
}

impl TryFrom<&String> for Reg {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(Reg::from_str(value)?)
    }
}

impl From<&Reg> for u64 {
    fn from(value: &Reg) -> Self {
        match value {
            Reg::Zero => 0x00,
            Reg::Ra => 0x01,
            Reg::Sp => 0x02,
            Reg::Gp => 0x03,
            Reg::Tp => 0x04,
            Reg::T0 => 0x05,
            Reg::T1 => 0x06,
            Reg::T2 => 0x07,
            Reg::S0 => 0x08,
            Reg::S1 => 0x09,
            Reg::A0 => 0x0A,
            Reg::A1 => 0x0B,
            Reg::A2 => 0x0C,
            Reg::A3 => 0x0D,
            Reg::A4 => 0x0E,
            Reg::A5 => 0x0F,
            Reg::A6 => 0x10,
            Reg::A7 => 0x11,
            Reg::S2 => 0x12,
            Reg::S3 => 0x13,
            Reg::S4 => 0x14,
            Reg::S5 => 0x15,
            Reg::S6 => 0x16,
            Reg::S7 => 0x17,
            Reg::S8 => 0x18,
            Reg::S9 => 0x19,
            Reg::S10 => 0x1A,
            Reg::S11 => 0x1B,
            Reg::T3 => 0x1C,
            Reg::T4 => 0x1D,
            Reg::T5 => 0x1E,
            Reg::T6 => 0x1F,
            Reg::Mstatus => 0x300,
            Reg::Mepc => 0x341,
            Reg::Mtvec => 0x305,
            Reg::Mcause => 0x342,
        }
    }
}

impl Into<u64> for Reg {
    fn into(self) -> u64 {
        match self {
            Reg::Zero => 0x00,
            Reg::Ra => 0x01,
            Reg::Sp => 0x02,
            Reg::Gp => 0x03,
            Reg::Tp => 0x04,
            Reg::T0 => 0x05,
            Reg::T1 => 0x06,
            Reg::T2 => 0x07,
            Reg::S0 => 0x08,
            Reg::S1 => 0x09,
            Reg::A0 => 0x0A,
            Reg::A1 => 0x0B,
            Reg::A2 => 0x0C,
            Reg::A3 => 0x0D,
            Reg::A4 => 0x0E,
            Reg::A5 => 0x0F,
            Reg::A6 => 0x10,
            Reg::A7 => 0x11,
            Reg::S2 => 0x12,
            Reg::S3 => 0x13,
            Reg::S4 => 0x14,
            Reg::S5 => 0x15,
            Reg::S6 => 0x16,
            Reg::S7 => 0x17,
            Reg::S8 => 0x18,
            Reg::S9 => 0x19,
            Reg::S10 => 0x1A,
            Reg::S11 => 0x1B,
            Reg::T3 => 0x1C,
            Reg::T4 => 0x1D,
            Reg::T5 => 0x1E,
            Reg::T6 => 0x1F,
            Reg::Mstatus => 0x300,
            Reg::Mepc => 0x341,
            Reg::Mtvec => 0x305,
            Reg::Mcause => 0x342,
        }
    }
}
