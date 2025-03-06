#[macro_export]
macro_rules! err {
    ($($t:tt)*) => {{
        let s = format!($($t)*);
        Err(s)
    }};
}

#[macro_export]
macro_rules! src {
    {} => 
    { $crate::src(
        option_env!("CARGO_PKG_NAME").map(|s| s.to_string()),
        file!(),
        line!(),
    )};
}

fn src (
    crate_name: Option<String>,
    file: &str,
    line: u32) -> String 
{
    match &crate_name {
        Some(crate_name) => format!("[{}:{}:{}]", crate_name, file, line),
        None => format!("[unknown:{}:{}]", file, line),
    }
}


