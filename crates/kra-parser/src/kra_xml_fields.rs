use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use strong_xml::{XmlError, XmlRead, XmlReader, XmlResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
pub struct KraXmlValue<T> {
    pub value: T,
}

#[derive(Debug)]
pub struct KraXmlTimeRange<T> {
    pub from: T,
    pub to: T,
}

#[derive(Debug)]
pub struct KraXmlPoint<T> {
    pub x: T,
    pub y: T,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct ParsedXmlTag {
    tag_name: String,
    tag_attributes: HashMap<String, String>,
}

impl ParsedXmlTag {
    fn from_reader(reader: &mut XmlReader<'_>) -> XmlResult<Self> {
        let tag_name = reader.find_element_start(None)?.unwrap().to_string();
        reader.read_till_element_start(&tag_name)?;

        let mut tag_attributes = HashMap::new();

        while let Ok(Some((key, value))) = reader.find_attribute() {
            tag_attributes.insert(key.to_string(), value.to_string());
        }

        reader.read_to_end(&tag_name)?;

        Ok(ParsedXmlTag { tag_name, tag_attributes })
    }

    #[rustfmt::skip]
    fn attribute<T>(&self, attribute_name: &str) -> XmlResult<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug + Error + Sync + Send + 'static,
    {
        if let Some(value) = self.tag_attributes.get(attribute_name) {
            Ok(T::from_str(value).map_err(|err| XmlError::FromStr(Box::new(err)))?)
        } else {
            Err(XmlError::MissingField {
                name: self.tag_name.clone(),
                field: attribute_name.to_owned(),
            })
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[rustfmt::skip]
impl<T> XmlRead<'_> for KraXmlValue<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + Error + Sync + Send + 'static,
{
    fn from_reader(reader: &mut XmlReader<'_>) -> XmlResult<Self> {
        let xml_tag = ParsedXmlTag::from_reader(reader)?;

        if xml_tag.attribute::<String>("type")? == "value" {
            Ok(KraXmlValue {
                value: xml_tag.attribute::<T>("value")?,
            })
        } else {
            Err(XmlError::UnexpectedEof)
        }
    }
}

#[rustfmt::skip]
impl<T> XmlRead<'_> for KraXmlTimeRange<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + Error + Sync + Send + 'static,
{
    fn from_reader(reader: &mut XmlReader<'_>) -> XmlResult<Self> {
        let xml_tag = ParsedXmlTag::from_reader(reader)?;

        if xml_tag.attribute::<String>("type")? == "timerange" {
            Ok(KraXmlTimeRange {
                from: xml_tag.attribute::<T>("from")?,
                to: xml_tag.attribute::<T>("to")?,
            })
        } else {
            Err(XmlError::UnexpectedEof)
        }
    }
}

#[rustfmt::skip]
impl<T> XmlRead<'_> for KraXmlPoint<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + Error + Sync + Send + 'static,
{
    fn from_reader(reader: &mut XmlReader<'_>) -> XmlResult<Self> {
        let xml_tag = ParsedXmlTag::from_reader(reader)?;

        if ["point", "pointf"].contains(&xml_tag.attribute::<String>("type")?.as_str()) {
            Ok(KraXmlPoint {
                x: xml_tag.attribute::<T>("x")?,
                y: xml_tag.attribute::<T>("y")?,
            })
        } else {
            Err(XmlError::UnexpectedEof)
        }
    }
}
