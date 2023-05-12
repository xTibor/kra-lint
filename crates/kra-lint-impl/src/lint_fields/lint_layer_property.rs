use serde::{Deserialize, Serialize};

use kra_parser::kra_maindoc::KraMainDocLayer;
use kra_parser::kra_utils::KraLayerType;

use crate::LintError;

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum LintLayerProperty<T> {
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
    pub(crate) fn get<'a>(&'a self, layer: &KraMainDocLayer) -> Result<(&'a Option<T>, &str), LintError> {
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
            } => match layer.layer_type()? {
                KraLayerType::PaintLayer  => Ok((paint_layers,  "paint layer" )),
                KraLayerType::GroupLayer  => Ok((group_layers,  "group layer" )),
                KraLayerType::CloneLayer  => Ok((clone_layers,  "clone layer" )),
                KraLayerType::VectorLayer => Ok((vector_layers, "vector layer")),
                KraLayerType::FilterLayer => Ok((filter_layers, "filter layer")),
                KraLayerType::FillLayer   => Ok((fill_layers,   "fill layer"  )),
                KraLayerType::FileLayer   => Ok((file_layers,   "file layer"  )),
            },
        }
    }
}
