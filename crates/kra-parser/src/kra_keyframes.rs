use derive_more::IntoIterator;
use serde::{Deserialize, Serialize};
use strong_xml::XmlRead;
use strum::{Display, EnumString};

use crate::kra_color_label::KraColorLabel;
use crate::kra_xml_fields::KraXmlPoint;

#[derive(Debug, XmlRead, IntoIterator)]
#[xml(tag = "keyframes")]
pub struct KraKeyframesDocument {
    #[into_iterator(ref)]
    #[xml(child = "channel")]
    pub channels: Vec<KraKeyframeChannel>,
}

#[derive(Debug, XmlRead, IntoIterator)]
#[xml(tag = "channel")]
pub struct KraKeyframeChannel {
    #[xml(attr = "name")]
    pub name: String,

    #[into_iterator(ref)]
    #[xml(child = "keyframe")]
    pub keyframes: Vec<KraKeyframe>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "keyframe")]
pub struct KraKeyframe {
    #[xml(attr = "time")]
    pub time: usize,

    #[xml(attr = "color-label")]
    pub color_label: KraColorLabel,

    #[xml(attr = "interpolation")]
    pub interpolation: Option<KraKeyframeInterpolation>,

    #[xml(attr = "tangents")]
    pub tangents: Option<KraKeyframeTangents>,

    #[xml(attr = "value")]
    pub value: Option<f64>,

    #[xml(attr = "frame")]
    pub frame: Option<String>,

    #[xml(child = "leftTangent")]
    pub left_tangent: Option<KraXmlPoint<f64>>,

    #[xml(child = "rightTangent")]
    pub right_tangent: Option<KraXmlPoint<f64>>,

    #[xml(child = "offset")]
    pub offset: Option<KraXmlPoint<isize>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum KraKeyframeInterpolation {
    #[strum(serialize = "constant")]
    Constant,

    #[strum(serialize = "linear")]
    Linear,

    #[strum(serialize = "bezier")]
    Bezier,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum KraKeyframeTangents {
    #[strum(serialize = "smooth")]
    Smooth,

    #[strum(serialize = "sharp")]
    Sharp,
}
