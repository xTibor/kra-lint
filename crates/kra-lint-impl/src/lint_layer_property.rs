use serde::Deserialize;

use kra_parser::kra_maindoc::KraMainDocLayer;

use crate::LintError;

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum LintLayerProperty<T> {
    All(Option<T>),
    ByType {
        paint_layers:  Option<T>,
        group_layers:  Option<T>,
        clone_layers:  Option<T>,
        vector_layers: Option<T>,
        filter_layers: Option<T>,
        fill_layers:   Option<T>,
        file_layers:   Option<T>,
    },
}

impl<T> LintLayerProperty<T> {
    #[rustfmt::skip]
    pub fn get<'a>(&'a self, layer: &KraMainDocLayer) -> Result<(&'a Option<T>, &str), LintError> {
        match *self {
            LintLayerProperty::All(ref all_layers) => Ok((all_layers, "layer")),
            LintLayerProperty::ByType {
                ref paint_layers,
                ref group_layers,
                ref clone_layers,
                ref vector_layers,
                ref filter_layers,
                ref fill_layers,
                ref file_layers,
            } => match layer.node_type.as_str() {
                "paintlayer"      => Ok((paint_layers,  "paint layer" )),
                "grouplayer"      => Ok((group_layers,  "group layer" )),
                "clonelayer"      => Ok((clone_layers,  "clone layer" )),
                "shapelayer"      => Ok((vector_layers, "vector layer")),
                "adjustmentlayer" => Ok((filter_layers, "filter layer")),
                "generatorlayer"  => Ok((fill_layers,   "fill layer"  )),
                "filelayer"       => Ok((file_layers,   "file layer"  )),
                _ => Err(LintError::UnknownLayerNodeType(layer.node_type.clone())),
            },
        }
    }
}
