use masterror::Error;

#[derive(Debug, Error)]
#[error("{value}")]
struct DisplayOnly {
    value: &'static str,
}

#[derive(Debug, Error)]
#[error("{value:?} {value:#?}")]
struct DebugPair {
    value: &'static str,
}

#[derive(Debug, Error)]
#[error("{value:x} {value:#x}")]
struct LowerHexPair {
    value: u32,
}

#[derive(Debug, Error)]
#[error("{value:X} {value:#X}")]
struct UpperHexPair {
    value: u32,
}

#[derive(Debug, Error)]
#[error("{value:b} {value:#b}")]
struct BinaryPair {
    value: u16,
}

#[derive(Debug, Error)]
#[error("{value:o} {value:#o}")]
struct OctalPair {
    value: u16,
}

#[derive(Debug, Error)]
#[error("{value:e} {value:#e}")]
struct LowerExpPair {
    value: f64,
}

#[derive(Debug, Error)]
#[error("{value:E} {value:#E}")]
struct UpperExpPair {
    value: f64,
}

#[derive(Debug, Error)]
#[error("{value:p} {value:#p}")]
struct PointerPair {
    value: *const u32,
}

fn main() {
    let _ = DisplayOnly { value: "display" }.to_string();
    let _ = DebugPair { value: "debug" }.to_string();
    let _ = LowerHexPair { value: 0x5A5Au32 }.to_string();
    let _ = UpperHexPair { value: 0x5A5Au32 }.to_string();
    let _ = BinaryPair { value: 0b1010_1100u16 }.to_string();
    let _ = OctalPair { value: 0b1010_1100u16 }.to_string();
    let _ = LowerExpPair { value: 1234.5 }.to_string();
    let _ = UpperExpPair { value: 1234.5 }.to_string();
    let _ = PointerPair {
        value: core::ptr::null::<u32>()
    }
    .to_string();
}
