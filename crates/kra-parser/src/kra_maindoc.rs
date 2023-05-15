use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strong_xml::XmlRead;

use crate::kra_error::KraError;

#[derive(Debug, XmlRead)]
#[xml(tag = "DOC")]
pub struct KraMainDoc {
    #[xml(attr = "kritaVersion")]
    pub software_version: String,

    #[xml(attr = "editor")]
    pub editor: String,

    #[xml(attr = "syntaxVersion")]
    pub syntax_version: String,

    #[xml(child = "IMAGE")]
    pub image: KraMainDocImage,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "IMAGE")]
pub struct KraMainDocImage {
    #[xml(attr = "name")]
    pub name: String,

    #[xml(attr = "description")]
    pub description: String,

    #[xml(attr = "mime")]
    pub mime: String,

    #[xml(attr = "width")]
    pub width: usize,

    #[xml(attr = "height")]
    pub height: usize,

    #[xml(attr = "x-res")]
    pub x_res: f64,

    #[xml(attr = "y-res")]
    pub y_res: f64,

    #[xml(attr = "profile")]
    pub profile: String,

    #[xml(attr = "colorspacename")]
    pub colorspace_name: String,

    #[xml(child = "layers")]
    pub layer_container: KraMainDocLayerContainer,

    #[xml(child = "Palettes")]
    pub palette_container: Option<KraMainDocPaletteContainer>,

    #[xml(child = "compositions")]
    pub composition_container: Option<KraMainDocCompositionContainer>,

    #[xml(child = "animation")]
    pub animation: KraMainDocAnimation,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "layers")]
pub struct KraMainDocLayerContainer {
    #[xml(child = "layer")]
    pub layers: Vec<KraMainDocLayer>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "layer")]
pub struct KraMainDocLayer {
    #[xml(attr = "channelflags")]
    pub channel_flags: String,

    #[xml(attr = "channellockflags")]
    pub channel_lock_flags: Option<String>,

    #[xml(attr = "collapsed")]
    pub collapsed: usize,

    #[xml(attr = "colorlabel")]
    pub color_label: KraColorLabel,

    #[xml(attr = "colorspacename")]
    pub colorspace_name: Option<String>,

    #[xml(attr = "compositeop")]
    pub composite_op: String,

    #[xml(attr = "filename")]
    pub file_name: String,

    #[xml(attr = "generatorname")]
    pub generator_name: Option<String>,

    #[xml(attr = "generatorversion")]
    pub generator_version: Option<usize>,

    #[xml(attr = "intimeline")]
    pub in_timeline: usize,

    #[xml(attr = "locked")]
    pub locked: usize,

    #[xml(attr = "name")]
    pub name: String,

    #[xml(attr = "nodetype")]
    pub layer_type: KraLayerType,

    #[xml(attr = "onionskin")]
    pub onion_skin: Option<String>,

    #[xml(attr = "opacity")]
    pub opacity: usize,

    #[xml(attr = "selected")]
    pub selected: Option<String>,

    #[xml(attr = "uuid")]
    pub uuid: String,

    #[xml(attr = "visible")]
    pub visible: usize,

    #[xml(attr = "x")]
    pub x: isize,

    #[xml(attr = "y")]
    pub y: isize,

    #[xml(attr = "clonetype")]
    pub clone_type: Option<usize>,

    #[xml(attr = "clonefrom")]
    pub clone_from: Option<String>,

    #[xml(attr = "clonefromuuid")]
    pub clone_from_uuid: Option<String>,

    #[xml(attr = "source")]
    pub source: Option<String>,

    #[xml(attr = "keyframes")]
    pub keyframes: Option<String>,

    #[xml(attr = "layerstyle")]
    pub layer_style: Option<String>,

    #[xml(child = "layers")]
    pub layer_container: Option<KraMainDocLayerContainer>,

    #[xml(child = "masks")]
    pub mask_container: Option<KraMainDocMaskContainer>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "animation")]
pub struct KraMainDocAnimation {
    #[xml(child = "framerate")]
    pub framerate: KraMainDocAnimationFramerate,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "framerate")]
pub struct KraMainDocAnimationFramerate {
    #[xml(attr = "value")]
    pub value: usize,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "masks")]
pub struct KraMainDocMaskContainer {
    #[xml(child = "mask")]
    pub masks: Vec<KraMainDocMask>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "mask")]
pub struct KraMainDocMask {
    #[xml(attr = "active")]
    pub active: Option<usize>,

    #[xml(attr = "cleanup")]
    pub cleanup: Option<usize>,

    #[xml(attr = "colorlabel")]
    pub color_label: Option<KraColorLabel>,

    #[xml(attr = "colorspacename")]
    pub colorspace_name: Option<String>,

    #[xml(attr = "compositeop")]
    pub composite_op: Option<String>,

    #[xml(attr = "edge-detection-size")]
    pub edge_detection_size: Option<usize>,

    #[xml(attr = "edit-keystrokes")]
    pub edit_keystrokes: Option<usize>,

    #[xml(attr = "filename")]
    pub file_name: Option<String>,

    #[xml(attr = "filtername")]
    pub filter_name: Option<String>,

    #[xml(attr = "filterversion")]
    pub filter_version: Option<usize>,

    #[xml(attr = "fuzzy-radius")]
    pub fuzzy_radius: Option<usize>,

    #[xml(attr = "intimeline")]
    pub in_timeline: Option<usize>,

    #[xml(attr = "limit-to-device")]
    pub limit_to_device: Option<usize>,

    #[xml(attr = "locked")]
    pub locked: usize,

    #[xml(attr = "name")]
    pub name: String,

    #[xml(attr = "nodetype")]
    pub mask_type: KraMaskType,

    #[xml(attr = "show-coloring")]
    pub show_coloring: Option<usize>,

    #[xml(attr = "use-edge-detection")]
    pub use_edge_detection: Option<usize>,

    #[xml(attr = "uuid")]
    pub uuid: String,

    #[xml(attr = "visible")]
    pub visible: usize,

    #[xml(attr = "x")]
    pub x: usize,

    #[xml(attr = "y")]
    pub y: usize,

    #[xml(attr = "keyframes")]
    pub keyframes: Option<String>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Palettes")]
pub struct KraMainDocPaletteContainer {
    #[xml(child = "resource")]
    pub resources: Vec<KraMainDocResource>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "resource")]
pub struct KraMainDocResource {
    #[xml(attr = "filename")]
    pub file_name: String,

    #[xml(attr = "md5sum")]
    pub md5_sum: String,

    #[xml(attr = "name")]
    pub name: String,

    #[xml(attr = "type")]
    pub r#type: String,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "compositions")]
pub struct KraMainDocCompositionContainer {
    #[xml(child = "composition")]
    pub compositions: Vec<KraMainDocComposition>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "composition")]
pub struct KraMainDocComposition {
    #[xml(attr = "exportEnabled")]
    pub export_enabled: usize,

    #[xml(attr = "name")]
    pub name: String,

    #[xml(child = "value")]
    pub values: Vec<KraMainDocCompositionValue>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "value")]
pub struct KraMainDocCompositionValue {
    #[xml(attr = "collapsed")]
    pub collapsed: usize,

    #[xml(attr = "visible")]
    pub visible: usize,

    #[xml(attr = "uuid")]
    pub uuid: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KraLayerType {
    PaintLayer,
    GroupLayer,
    CloneLayer,
    VectorLayer,
    FilterLayer,
    FillLayer,
    FileLayer,
}

impl FromStr for KraLayerType {
    type Err = KraError;

    #[rustfmt::skip]
    fn from_str(layer_type: &str) -> Result<KraLayerType, KraError> {
        match layer_type {
            "paintlayer"      => Ok(KraLayerType::PaintLayer ),
            "grouplayer"      => Ok(KraLayerType::GroupLayer ),
            "clonelayer"      => Ok(KraLayerType::CloneLayer ),
            "shapelayer"      => Ok(KraLayerType::VectorLayer),
            "adjustmentlayer" => Ok(KraLayerType::FilterLayer),
            "generatorlayer"  => Ok(KraLayerType::FillLayer  ),
            "filelayer"       => Ok(KraLayerType::FileLayer  ),
            _ => Err(KraError::UnknownLayerNodeType(layer_type.to_owned())),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KraMaskType {
    TransparencyMask,
    FilterMask,
    ColorizeMask,
    TransformMask,
    LocalSelection,
}

impl FromStr for KraMaskType {
    type Err = KraError;

    #[rustfmt::skip]
    fn from_str(mask_type: &str) -> Result<KraMaskType, KraError> {
        match mask_type {
            "transparencymask" => Ok(KraMaskType::TransparencyMask),
            "filtermask"       => Ok(KraMaskType::FilterMask      ),
            "colorizemask"     => Ok(KraMaskType::ColorizeMask    ),
            "transformmask"    => Ok(KraMaskType::TransformMask   ),
            "selectionmask"    => Ok(KraMaskType::LocalSelection  ),
            _ => Err(KraError::UnknownMaskNodeType(mask_type.to_owned())),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KraColorLabel {
    None,
    Blue,
    Green,
    Yellow,
    Orange,
    Brown,
    Red,
    Purple,
    Black,
}

impl FromStr for KraColorLabel {
    type Err = KraError;

    #[rustfmt::skip]
    fn from_str(color_label: &str) -> Result<KraColorLabel, KraError> {
        match color_label {
            "0" => Ok(KraColorLabel::None  ),
            "1" => Ok(KraColorLabel::Blue  ),
            "2" => Ok(KraColorLabel::Green ),
            "3" => Ok(KraColorLabel::Yellow),
            "4" => Ok(KraColorLabel::Orange),
            "5" => Ok(KraColorLabel::Brown ),
            "6" => Ok(KraColorLabel::Red   ),
            "7" => Ok(KraColorLabel::Purple),
            "8" => Ok(KraColorLabel::Black ),
            _ => Err(KraError::UnknownColorLabel(color_label.to_owned())),
        }
    }
}
