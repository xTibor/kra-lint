use serde::{Deserialize, Serialize};

use kra_parser::kra_main_doc::{KraMainDocMask, KraMaskType};

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum ValueByMaskType<T> {
    All(Option<T>),
    ByType {
        transparency_masks: Option<T>,
        filter_masks:       Option<T>,
        colorize_masks:     Option<T>,
        transform_masks:    Option<T>,
        local_selections:   Option<T>,
    },
}

impl<T> ValueByMaskType<T> {
    #[rustfmt::skip]
    pub(crate) fn get<'a>(&'a self, mask: &KraMainDocMask) -> (&'a Option<T>, &str) {
        match *self {
            ValueByMaskType::All(ref all_masks) => (all_masks, "mask"),
            ValueByMaskType::ByType {
                ref transparency_masks,
                ref filter_masks,
                ref colorize_masks,
                ref transform_masks,
                ref local_selections,
            } => match mask.mask_type {
                KraMaskType::TransparencyMask => (transparency_masks, "transparency mask"),
                KraMaskType::FilterMask       => (filter_masks,       "filter mask"      ),
                KraMaskType::ColorizeMask     => (colorize_masks,     "colorize mask"    ),
                KraMaskType::TransformMask    => (transform_masks,    "transform mask"   ),
                KraMaskType::LocalSelection   => (local_selections,   "local selection"  ),
            },
        }
    }
}
