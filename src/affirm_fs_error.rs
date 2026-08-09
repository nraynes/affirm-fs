use rust_alert::alert;

/// A custom error type used to convert error types from various crates.
#[derive(Eq, PartialOrd, Ord)]
#[alert(errors = [
    String,
    std::io::Error,
])]
pub struct AffirmFsError {}
