use serde::Deserialize;

use crate::lints::{
    LintLayerTypeFlags, LintMaskTypeFlags, LintPass, LintPassResult,
};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassAnimation {
    pub animated_layers: LintLayerTypeFlags<bool>,
    pub animated_masks: LintMaskTypeFlags<bool>,
    pub framerate: Option<usize>,
}

impl LintPass for LintPassAnimation {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                let (layer_opt, layer_display) =
                    self.animated_layers.get(layer);

                #[allow(clippy::collapsible_if)]
                if *layer_opt == Some(false) {
                    if layer.keyframes.is_some() {
                        results.push(format!(
                            "Prohibited use of animated {} (layer: \"{}\")",
                            layer_display, layer.name
                        ));
                    }
                }
            }
        }

        // Sub-pass #2
        {
            for (layer, mask) in kra_archive.all_masks() {
                let (mask_opt, mask_display) = self.animated_masks.get(mask);

                #[allow(clippy::collapsible_if)]
                if *mask_opt == Some(false) {
                    if mask.keyframes.is_some() {
                        results.push(format!(
                            "Prohibited use of animated {} (layer: \"{}\", mask: \"{}\")",
                            mask_display, layer.name, mask.name
                        ));
                    }
                }
            }
        }

        // Sub-pass #3
        {
            if let Some(framerate) = self.framerate {
                let kra_framerate =
                    kra_archive.main_doc.image.animation.framerate.value;

                if kra_framerate != framerate {
                    results.push(format!(
                        "Incorrect animation framerate (expected: {}fps, found: {}fps)",
                        framerate, kra_framerate
                    ));
                }
            }
        }

        results
    }
}
