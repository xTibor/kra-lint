use serde::{Deserialize, Serialize};
use svg::node::element::tag::Type;
use svg::parser::Event;

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::KraLayerType;

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::LintMessages;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassVectorLayers {
    font_family: Option<LintStringMatchExpression>,
    stroke_linecap: Option<LintStringMatchExpression>,
    stroke_linejoin: Option<LintStringMatchExpression>,
    placeholder_text: Option<LintStringMatchExpression>,
}

impl LintPass for LintPassVectorLayers {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1, #2, #3, #4
        {
            for layer in kra_archive.all_layers_by_type(KraLayerType::VectorLayer) {
                let content_svg_data = layer.content_svg(kra_archive)?;
                let content_svg_parser = svg::read(&content_svg_data)?;

                for svg_event in content_svg_parser {
                    // Sub-pass #1
                    if let Some(font_family) = self.font_family.as_ref() {
                        if let Event::Tag("text" | "tspan", Type::Start, svg_attributes) = &svg_event {
                            if let Some(svg_font_family) = svg_attributes.get("font-family") {
                                if !font_family.matches(svg_font_family) {
                                    lint_messages.push(
                                        "Prohibited font family on vector layer",
                                        format!(
                                            "Layer: \"{}\", Expected: {}, Found: \"{}\"",
                                            layer.name.escape_debug(),
                                            font_family,
                                            svg_font_family.escape_debug()
                                        ),
                                    );
                                }
                            }
                        }
                    }

                    // Sub-pass #2
                    if let Some(stroke_linecap) = self.stroke_linecap.as_ref() {
                        if let Event::Tag("rect" | "ellipse" | "path" | "text", Type::Start, svg_attributes) =
                            &svg_event
                        {
                            if let Some(svg_stroke_linecap) = svg_attributes.get("stroke-linecap") {
                                if !stroke_linecap.matches(svg_stroke_linecap) {
                                    lint_messages.push(
                                        "Prohibited stroke line cap on vector layer",
                                        format!(
                                            "Layer: \"{}\", Expected: {}, Found: \"{}\"",
                                            layer.name.escape_debug(),
                                            stroke_linecap,
                                            svg_stroke_linecap.escape_debug()
                                        ),
                                    );
                                }
                            }
                        }
                    }

                    // Sub-pass #3
                    if let Some(stroke_linejoin) = self.stroke_linejoin.as_ref() {
                        if let Event::Tag("rect" | "ellipse" | "path" | "text", Type::Start, svg_attributes) =
                            &svg_event
                        {
                            if let Some(svg_stroke_linejoin) = svg_attributes.get("stroke-linejoin") {
                                if !stroke_linejoin.matches(svg_stroke_linejoin) {
                                    lint_messages.push(
                                        "Prohibited stroke line join on vector layer",
                                        format!(
                                            "Layer: \"{}\", Expected: {}, Found: \"{}\"",
                                            layer.name.escape_debug(),
                                            stroke_linejoin,
                                            svg_stroke_linejoin.escape_debug()
                                        ),
                                    );
                                }
                            }
                        }
                    }

                    // Sub-pass #4
                    if let Some(placeholder_text) = self.placeholder_text.as_ref() {
                        if let Event::Text(svg_text) = &svg_event {
                            if placeholder_text.matches(svg_text) {
                                lint_messages.push(
                                    "Prohibited placeholder text on vector layer",
                                    format!(
                                        "Layer: \"{}\", Placeholder text: \"{}\"",
                                        layer.name.escape_debug(),
                                        svg_text.escape_debug()
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
