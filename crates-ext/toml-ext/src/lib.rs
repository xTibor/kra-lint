use derive_more::{Display, Error, From};

#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    #[display(fmt = "TOML I/O error")]
    IoError(std::io::Error),

    #[from]
    #[display(fmt = "TOML deserialization error")]
    TomlDeError(toml::de::Error),

    #[from]
    #[display(fmt = "TOML serialization error")]
    TomlSerError(toml::ser::Error),
}

pub fn from_reader<R, T>(reader: R) -> Result<T, Error>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    Ok(toml::from_str(&std::io::read_to_string(reader)?)?)
}

pub fn to_writer<W, T>(mut writer: W, value: &T) -> Result<(), Error>
where
    W: std::io::Write,
    T: ?Sized + serde::Serialize,
{
    let tmp_string = toml::ser::to_string_pretty(value)?;
    Ok(writer.write_all(tmp_string.as_bytes())?)
}
