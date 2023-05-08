use serde::Deserialize;

use kra_parser::kra_maindoc::KraMainDocMask;

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum LintMaskProperty<T> {
    All(Option<T>),
    ByType {
        transparency_masks: Option<T>,
        filter_masks:       Option<T>,
        colorize_masks:     Option<T>,
        transform_masks:    Option<T>,
        local_selections:   Option<T>,
    },
}

impl<T> LintMaskProperty<T> {
    #[rustfmt::skip]
    pub fn get<'a>(&'a self, mask: &KraMainDocMask) -> (&'a Option<T>, &str) {
        match *self {
            LintMaskProperty::All(ref all_masks) => (all_masks, "mask"),
            LintMaskProperty::ByType {
                ref transparency_masks,
                ref filter_masks,
                ref colorize_masks,
                ref transform_masks,
                ref local_selections,
            } => match mask.node_type.as_str() {
                "transparencymask" => (transparency_masks, "transparency mask"),
                "filtermask"       => (filter_masks,       "filter mask"      ),
                "colorizemask"     => (colorize_masks,     "colorize mask"    ),
                "transformmask"    => (transform_masks,    "transform mask"   ),
                "selectionmask"    => (local_selections,   "local selection"  ),
                _ => unreachable!("Unknown mask node type: \"{}\"", mask.node_type),
            },
        }
    }
}
