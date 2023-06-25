use std::collections::HashMap;
use std::str::FromStr;

use strong_xml::{XmlRead, XmlReader, XmlResult};

#[derive(Debug)]
pub struct KraXmlValue<T> {
    pub value: T,
}

#[derive(Debug)]
pub struct KraXmlTimeRange<T> {
    pub from: T,
    pub to: T,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn read_xml_tag_attributes(reader: &mut XmlReader<'_>) -> XmlResult<HashMap<String, String>> {
    let tag_name = reader.find_element_start(None)?.unwrap();
    reader.read_till_element_start(tag_name)?;

    let mut results = HashMap::new();

    while let Ok(Some((key, value))) = reader.find_attribute() {
        results.insert(key.to_string(), value.to_string());
    }

    reader.read_to_end(tag_name)?;
    Ok(results)
}

impl<T> XmlRead<'_> for KraXmlValue<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from_reader(reader: &mut XmlReader<'_>) -> XmlResult<Self> {
        let attributes = read_xml_tag_attributes(reader)?;

        if attributes.get("type").cloned().as_deref() == Some("value") {
            Ok(KraXmlValue { value: T::from_str(attributes.get("value").unwrap()).unwrap() })
        } else {
            Err(strong_xml::XmlError::UnexpectedEof)
        }
    }
}

impl<T> XmlRead<'_> for KraXmlTimeRange<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from_reader(reader: &mut XmlReader<'_>) -> XmlResult<Self> {
        let attributes = read_xml_tag_attributes(reader)?;

        if attributes.get("type").cloned().as_deref() == Some("timerange") {
            Ok(KraXmlTimeRange {
                from: T::from_str(attributes.get("from").unwrap()).unwrap(),
                to: T::from_str(attributes.get("to").unwrap()).unwrap(),
            })
        } else {
            Err(strong_xml::XmlError::UnexpectedEof)
        }
    }
}
