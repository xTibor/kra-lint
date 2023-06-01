use serde::{Deserialize, Serialize};
use svg::node::element::tag::Type;
use svg::parser::Event;

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::KraLayerType;

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_output::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_bug, meta_expected, meta_found, meta_layer};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassVectorLayers {
    font_family: Option<LintStringMatchExpression>,
    stroke_linecap: Option<LintStringMatchExpression>,
    stroke_linejoin: Option<LintStringMatchExpression>,
    placeholder_text: Option<LintStringMatchExpression>,
    warn_broken_text_gradients: Option<bool>,
}

impl LintPass for LintPassVectorLayers {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1, #2, #3, #4, #5
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
                                    #[rustfmt::skip]
                                    lint_messages.push(
                                        "Prohibited font family on vector layer",
                                        &[
                                            meta_layer!(layer),
                                            meta_expected!(font_family),
                                            meta_found!(svg_font_family),
                                        ],
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
                                    #[rustfmt::skip]
                                    lint_messages.push(
                                        "Prohibited stroke line cap on vector layer",
                                        &[
                                            meta_layer!(layer),
                                            meta_expected!(stroke_linecap),
                                            meta_found!(svg_stroke_linecap),
                                        ],
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
                                    #[rustfmt::skip]
                                    lint_messages.push(
                                        "Prohibited stroke line join on vector layer",
                                        &[
                                            meta_layer!(layer),
                                            meta_expected!(stroke_linejoin),
                                            meta_found!(svg_stroke_linejoin),
                                        ],
                                    );
                                }
                            }
                        }
                    }

                    // Sub-pass #4
                    if let Some(placeholder_text) = self.placeholder_text.as_ref() {
                        if let Event::Text(svg_text) = &svg_event {
                            if placeholder_text.matches(svg_text) {
                                #[rustfmt::skip]
                                lint_messages.push(
                                    "Prohibited placeholder text on vector layer",
                                    &[
                                        meta_layer!(layer),
                                        meta_found!(svg_text),
                                    ],
                                );
                            }
                        }
                    }

                    // Sub-pass #5
                    if self.warn_broken_text_gradients == Some(true) {
                        if let Event::Tag("text", Type::Start, svg_attributes) = &svg_event {
                            if let Some(svg_fill) = svg_attributes.get("fill") {
                                let compiled_regex = regex::Regex::new(r"^url\(#gradient\d+\)$")
                                    .expect("Failed to compile regular expression");

                                if compiled_regex.is_match(svg_fill) {
                                    #[rustfmt::skip]
                                    lint_messages.push(
                                        "Broken text gradient fill on vector layer",
                                        &[
                                            meta_layer!(layer),
                                            meta_bug!(430774),
                                        ],
                                    );
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
