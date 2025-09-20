use masterror::Error;

#[derive(Debug, Error)]
#[error(
    "display={pretty} debug={pretty:?} #debug={pretty:#?} x={value:x} X={value:X} \
     #x={value:#x} #X={value:#X} b={value:b} #b={value:#b} o={value:o} #o={value:#o} \
     e={float:e} #e={float:#e} E={float:E} #E={float:#E} p={ptr:p} #p={ptr:#p}"
)]
struct FormatterVariants {
    value: u32,
    float: f64,
    ptr:   *const u32,
    pretty: PrettyDebugValue,
}

#[derive(Debug)]
struct PrettyDebugValue {
    label: &'static str,
}

impl core::fmt::Display for PrettyDebugValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.label)
    }
}

fn main() {
    let showcase = FormatterVariants {
        value: 0x5A5Au32,
        float: 1234.5,
        ptr: core::ptr::null(),
        pretty: PrettyDebugValue { label: "alpha" },
    };

    let _ = showcase.to_string();
}
