use masterror::Error;

fn format_unit(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("unit")
}

fn format_pair(left: &i32, right: &i32, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "pair={left}:{right}")
}

fn format_struct_fields(
    count: &usize,
    label: &&'static str,
    f: &mut core::fmt::Formatter<'_>
) -> core::fmt::Result {
    write!(f, "struct={count}:{label}")
}

#[derive(Debug, Error)]
#[error(fmt = crate::format_struct_fields)]
struct StructFormatter {
    count: usize,
    label: &'static str,
}

#[derive(Debug, Error)]
enum EnumFormatter {
    #[error(fmt = crate::format_unit)]
    Unit,
    #[error(fmt = crate::format_pair)]
    Tuple(i32, i32),
    #[error(fmt = crate::format_pair)]
    Named { left: i32, right: i32 },
    #[error(fmt = crate::format_struct_fields)]
    Struct { count: usize, label: &'static str }
}

fn main() {
    let _ = StructFormatter {
        count: 1,
        label: "alpha"
    }
    .to_string();

    let _ = EnumFormatter::Unit.to_string();
    let _ = EnumFormatter::Tuple(10, 20).to_string();
    let _ = EnumFormatter::Named { left: 5, right: 15 }.to_string();
    let _ = EnumFormatter::Struct {
        count: 2,
        label: "beta"
    }
    .to_string();
}
