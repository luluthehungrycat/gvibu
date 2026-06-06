pub mod basename;
pub mod dirname;
pub mod echo_cmd;
pub mod false_cmd;
pub mod pwd_cmd;
pub mod true_cmd;

pub const TRUE: &[&str] = &["true", "gvibu-ref"];
pub const FALSE: &[&str] = &["false", "gvibu-ref"];
pub const ECHO: &[&str] = &["echo", "gvibu-ref"];
pub const PWD: &[&str] = &["pwd", "gvibu-ref"];
pub const BASENAME: &[&str] = &["basename", "gvibu-ref"];
pub const DIRNAME: &[&str] = &["dirname", "gvibu-ref"];
