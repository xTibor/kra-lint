use serde::Deserialize;

use crate::models::kra_maindoc::KraMainDocLayer;

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
    pub fn get<'a>(&'a self, layer: &KraMainDocLayer) -> (&'a Option<T>, &str) {
        match *self {
            LintLayerProperty::All(ref all_layers) => (all_layers, "layer"),
            LintLayerProperty::ByType {
                ref paint_layers,
                ref group_layers,
                ref clone_layers,
                ref vector_layers,
                ref filter_layers,
                ref fill_layers,
                ref file_layers,
            } => match layer.node_type.as_str() {
                "paintlayer"      => (paint_layers,  "paint layer" ),
                "grouplayer"      => (group_layers,  "group layer" ),
                "clonelayer"      => (clone_layers,  "clone layer" ),
                "shapelayer"      => (vector_layers, "vector layer"),
                "adjustmentlayer" => (filter_layers, "filter layer"),
                "generatorlayer"  => (fill_layers,   "fill layer"  ),
                "filelayer"       => (file_layers,   "file layer"  ),
                _ => unreachable!("Unknown layer node type: \"{}\"", layer.node_type),
            },
        }
    }
}
