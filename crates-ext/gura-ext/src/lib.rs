use derive_more::{Display, Error, From};

#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    #[display(fmt = "Gura I/O error")]
    IoError(std::io::Error),

    #[from]
    #[display(fmt = "Gura error")]
    GuraError(serde_gura::Error),
}

pub fn from_reader<R, T>(reader: R) -> Result<T, Error>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    Ok(serde_gura::from_str(&std::io::read_to_string(reader)?)?)
}

pub fn to_writer<W, T>(mut writer: W, value: &T) -> Result<(), Error>
where
    W: std::io::Write,
    T: serde::Serialize,
{
    let tmp_string = serde_gura::to_string(value)?;
    Ok(writer.write_all(tmp_string.as_bytes())?)
}
