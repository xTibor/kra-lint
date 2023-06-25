use derive_more::IntoIterator;
use serde::{Deserialize, Serialize};
use strong_xml::XmlRead;
use strum::{Display, EnumString};

use crate::kra_xml_fields::{KraXmlTimeRange, KraXmlValue};

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
    pub animation: Option<KraMainDocAnimation>,
}

#[rustfmt::skip]
#[derive(Debug, XmlRead, Default, IntoIterator)]
#[xml(tag = "layers")]
pub struct KraMainDocLayerContainer (
    #[into_iterator(ref)]
    #[xml(child = "layer")]
    pub Vec<KraMainDocLayer>,
);

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
    pub in_timeline: bool,

    #[xml(attr = "locked")]
    pub locked: bool,

    #[xml(attr = "name")]
    pub name: String,

    #[xml(attr = "nodetype")]
    pub layer_type: KraLayerType,

    #[xml(attr = "onionskin")]
    pub onion_skin: Option<bool>,

    #[xml(attr = "opacity")]
    pub opacity: usize,

    #[xml(attr = "selected")]
    pub selected: Option<String>,

    #[xml(attr = "uuid")]
    pub uuid: String,

    #[xml(attr = "visible")]
    pub visible: bool,

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

    #[xml(attr = "scalingmethod")]
    pub scaling_method: Option<KraScalingMethod>,

    #[xml(attr = "filtername")]
    pub filter_name: Option<String>,

    #[xml(attr = "filterversion")]
    pub filter_version: Option<usize>,

    #[xml(child = "layers")]
    pub layer_container: Option<KraMainDocLayerContainer>,

    #[xml(child = "masks")]
    pub mask_container: Option<KraMainDocMaskContainer>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "animation")]
pub struct KraMainDocAnimation {
    #[xml(child = "framerate")]
    pub framerate: KraXmlValue<usize>,

    #[xml(child = "range")]
    pub range: KraXmlTimeRange<usize>,

    #[xml(child = "currentTime")]
    pub current_time: KraXmlValue<usize>,
}

#[rustfmt::skip]
#[derive(Debug, XmlRead, Default, IntoIterator)]
#[xml(tag = "masks")]
pub struct KraMainDocMaskContainer (
    #[into_iterator(ref)]
    #[xml(child = "mask")]
    pub Vec<KraMainDocMask>,
);

#[derive(Debug, XmlRead)]
#[xml(tag = "mask")]
pub struct KraMainDocMask {
    #[xml(attr = "active")]
    pub active: Option<bool>,

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
    pub edit_keystrokes: Option<bool>,

    #[xml(attr = "filename")]
    pub file_name: Option<String>,

    #[xml(attr = "filtername")]
    pub filter_name: Option<String>,

    #[xml(attr = "filterversion")]
    pub filter_version: Option<usize>,

    #[xml(attr = "fuzzy-radius")]
    pub fuzzy_radius: Option<usize>,

    #[xml(attr = "intimeline")]
    pub in_timeline: Option<bool>,

    #[xml(attr = "limit-to-device")]
    pub limit_to_device: Option<bool>,

    #[xml(attr = "locked")]
    pub locked: bool,

    #[xml(attr = "name")]
    pub name: String,

    #[xml(attr = "nodetype")]
    pub mask_type: KraMaskType,

    #[xml(attr = "show-coloring")]
    pub show_coloring: Option<bool>,

    #[xml(attr = "use-edge-detection")]
    pub use_edge_detection: Option<bool>,

    #[xml(attr = "uuid")]
    pub uuid: String,

    #[xml(attr = "visible")]
    pub visible: bool,

    #[xml(attr = "x")]
    pub x: usize,

    #[xml(attr = "y")]
    pub y: usize,

    #[xml(attr = "keyframes")]
    pub keyframes: Option<String>,
}

#[rustfmt::skip]
#[derive(Debug, XmlRead, IntoIterator)]
#[xml(tag = "Palettes")]
pub struct KraMainDocPaletteContainer (
    #[into_iterator(ref)]
    #[xml(child = "resource")]
    pub Vec<KraMainDocResource>,
);

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

#[rustfmt::skip]
#[derive(Debug, XmlRead, IntoIterator)]
#[xml(tag = "compositions")]
pub struct KraMainDocCompositionContainer (
    #[into_iterator(ref)]
    #[xml(child = "composition")]
    pub Vec<KraMainDocComposition>,
);

#[derive(Debug, XmlRead)]
#[xml(tag = "composition")]
pub struct KraMainDocComposition {
    #[xml(attr = "exportEnabled")]
    pub export_enabled: bool,

    #[xml(attr = "name")]
    pub name: String,

    #[xml(child = "value")]
    pub values: Vec<KraMainDocCompositionValue>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "value")]
pub struct KraMainDocCompositionValue {
    #[xml(attr = "collapsed")]
    pub collapsed: bool,

    #[xml(attr = "visible")]
    pub visible: bool,

    #[xml(attr = "uuid")]
    pub uuid: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum KraLayerType {
    #[strum(serialize = "paintlayer", to_string = "paint_layer")]
    PaintLayer,

    #[strum(serialize = "grouplayer", to_string = "group_layer")]
    GroupLayer,

    #[strum(serialize = "clonelayer", to_string = "clone_layer")]
    CloneLayer,

    #[strum(serialize = "shapelayer", to_string = "vector_layer")]
    VectorLayer,

    #[strum(serialize = "adjustmentlayer", to_string = "filter_layer")]
    FilterLayer,

    #[strum(serialize = "generatorlayer", to_string = "fill_layer")]
    FillLayer,

    #[strum(serialize = "filelayer", to_string = "file_layer")]
    FileLayer,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum KraMaskType {
    #[strum(serialize = "transparencymask", to_string = "transparency_mask")]
    TransparencyMask,

    #[strum(serialize = "filtermask", to_string = "filter_mask")]
    FilterMask,

    #[strum(serialize = "colorizemask", to_string = "colorize_mask")]
    ColorizeMask,

    #[strum(serialize = "transformmask", to_string = "transform_mask")]
    TransformMask,

    #[strum(serialize = "selectionmask", to_string = "local_selection")]
    LocalSelection,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum KraColorLabel {
    #[strum(serialize = "0", to_string = "none")]
    None,

    #[strum(serialize = "1", to_string = "blue")]
    Blue,

    #[strum(serialize = "2", to_string = "green")]
    Green,

    #[strum(serialize = "3", to_string = "yellow")]
    Yellow,

    #[strum(serialize = "4", to_string = "orange")]
    Orange,

    #[strum(serialize = "5", to_string = "brown")]
    Brown,

    #[strum(serialize = "6", to_string = "red")]
    Red,

    #[strum(serialize = "7", to_string = "purple")]
    Purple,

    #[strum(serialize = "8", to_string = "black")]
    Black,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum KraScalingMethod {
    #[strum(serialize = "0", to_string = "none")]
    None,

    #[strum(serialize = "1", to_string = "scale_to_image")]
    ScaleToImage,

    #[strum(serialize = "2", to_string = "adapt_resolution")]
    AdaptResolution,
}
