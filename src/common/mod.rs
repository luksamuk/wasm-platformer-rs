//! Module for common operations and objects.

pub mod game_object;

macro_rules! println {
    ($fmt:expr) => (
        js!(console.log(@{format!(concat!($fmt, "\n"))}))
    );
    ($fmt:expr, $($arg:tt)*) => (
        js!(console.log(@{format!(concat!($fmt, "\n"), $($arg)*)}))
        
    );
}
