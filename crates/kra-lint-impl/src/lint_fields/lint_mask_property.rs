use serde::{Deserialize, Serialize};

use kra_parser::kra_maindoc::KraMainDocMask;
use kra_parser::kra_utils::KraMaskType;

use crate::LintError;

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum LintMaskProperty<T> {
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
    pub(crate) fn get<'a>(&'a self, mask: &KraMainDocMask) -> Result<(&'a Option<T>, &str), LintError> {
        match *self {
            LintMaskProperty::All(ref all_masks) => Ok((all_masks, "mask")),
            LintMaskProperty::ByType {
                ref transparency_masks,
                ref filter_masks,
                ref colorize_masks,
                ref transform_masks,
                ref local_selections,
            } => match mask.mask_type()? {
                KraMaskType::TransparencyMask => Ok((transparency_masks, "transparency mask")),
                KraMaskType::FilterMask       => Ok((filter_masks,       "filter mask"      )),
                KraMaskType::ColorizeMask     => Ok((colorize_masks,     "colorize mask"    )),
                KraMaskType::TransformMask    => Ok((transform_masks,    "transform mask"   )),
                KraMaskType::LocalSelection   => Ok((local_selections,   "local selection"  )),
            },
        }
    }
}
