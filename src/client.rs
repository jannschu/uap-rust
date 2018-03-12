use parser;

///`Client` struct, contains the parsed user agent information.
#[derive(Debug, PartialEq, Eq)]
pub struct Client {
    pub(crate) user_agent: String,
    pub browser: parser::Browser,
    pub os: parser::OS,
    pub device: parser::Device,
}
