use serde::Deserialize;

use kra_parser::kra_maindoc::KraMainDocMask;

use crate::LintError;

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
    pub fn get<'a>(&'a self, mask: &KraMainDocMask) -> Result<(&'a Option<T>, &str), LintError> {
        match *self {
            LintMaskProperty::All(ref all_masks) => Ok((all_masks, "mask")),
            LintMaskProperty::ByType {
                ref transparency_masks,
                ref filter_masks,
                ref colorize_masks,
                ref transform_masks,
                ref local_selections,
            } => match mask.node_type.as_str() {
                "transparencymask" => Ok((transparency_masks, "transparency mask")),
                "filtermask"       => Ok((filter_masks,       "filter mask"      )),
                "colorizemask"     => Ok((colorize_masks,     "colorize mask"    )),
                "transformmask"    => Ok((transform_masks,    "transform mask"   )),
                "selectionmask"    => Ok((local_selections,   "local selection"  )),
                _ => Err(LintError::UnknownMaskNodeType(mask.node_type.clone())),
            },
        }
    }
}
