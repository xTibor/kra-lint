use derive_more::IntoIterator;
use serde::Deserialize;
use strong_xml::XmlRead;
use strum::{EnumString};

#[derive(Debug, XmlRead, IntoIterator)]
#[xml(tag = "params")]
pub struct KraParamsContainer {
    #[xml(attr = "version")]
    pub version: usize,

    #[into_iterator(ref)]
    #[xml(child = "param")]
    pub params: Vec<KraParam>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "param")]
pub struct KraParam {
    #[xml(attr = "name")]
    pub name: String,

    #[xml(attr = "type")]
    pub r#type: Option<KraParamType>,

    #[xml(text)]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, EnumString)]
pub enum KraParamType {
    #[strum(serialize = "internal")]
    Internal,

    #[strum(serialize = "color")]
    Color,

    #[strum(serialize = "string")]
    String,

    #[strum(serialize = "bytearray")]
    ByteArray,
}
