#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    IOError(String),
}

impl From<reqwest_wasm::Error> for Error {
    fn from(value: reqwest_wasm::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("mp3-player error: {self:?}"))?;

        Ok(())
    }
}

impl std::error::Error for Error {}
