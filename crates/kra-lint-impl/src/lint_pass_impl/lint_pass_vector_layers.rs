use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_utils::KraLayerType;
use svg::node::element::tag::Type;
use svg::parser::Event;

use crate::lint_fields::LintStringMatchExpression;
use crate::{LintMessages, LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassVectorLayers {
    font_family: Option<LintStringMatchExpression>,
}

impl LintPass for LintPassVectorLayers {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(font_family) = self.font_family.as_ref() {
                for layer in kra_archive.all_layers() {
                    if layer.layer_type()? == KraLayerType::VectorLayer {
                        let content_svg_data = layer.content_svg(kra_archive)?;
                        let content_svg_parser = svg::read(&content_svg_data)?;

                        for svg_event in content_svg_parser {
                            if let Event::Tag("text" | "tspan", Type::Start, svg_attributes) = svg_event {
                                if let Some(svg_font_family) = svg_attributes.get("font-family") {
                                    if !font_family.matches(svg_font_family) {
                                        lint_messages.push(format!(
                                            "Prohibited font family on vector layer (layer: \"{}\", expected: {}, found: \"{}\")",
                                            layer.name, font_family, svg_font_family,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
