use serde::{Deserialize, Serialize};

use kra_parser::kra_main_doc::{KraLayerType, KraMainDocLayer};

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum ValueByLayerType<T> {
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

impl<T> ValueByLayerType<T> {
    #[rustfmt::skip]
    pub(crate) fn get<'a>(&'a self, layer: &KraMainDocLayer) -> (&'a Option<T>, &str) {
        match *self {
            ValueByLayerType::All(ref all_layers) => (all_layers, "layer"),
            ValueByLayerType::ByType {
                ref paint_layers,
                ref group_layers,
                ref clone_layers,
                ref vector_layers,
                ref filter_layers,
                ref fill_layers,
                ref file_layers,
            } => match layer.layer_type {
                KraLayerType::PaintLayer  => (paint_layers,  "paint layer" ),
                KraLayerType::GroupLayer  => (group_layers,  "group layer" ),
                KraLayerType::CloneLayer  => (clone_layers,  "clone layer" ),
                KraLayerType::VectorLayer => (vector_layers, "vector layer"),
                KraLayerType::FilterLayer => (filter_layers, "filter layer"),
                KraLayerType::FillLayer   => (fill_layers,   "fill layer"  ),
                KraLayerType::FileLayer   => (file_layers,   "file layer"  ),
            },
        }
    }
}
