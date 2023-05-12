use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_utils::KraLayerType;
use svg::node::element::tag::Type;
use svg::parser::Event;

use crate::lint_fields::LintStringMatchExpression;
use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassVectorLayers {
    font_family: Option<LintStringMatchExpression>,
}

impl LintPass for LintPassVectorLayers {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(font_family) = self.font_family.as_ref() {
                let mut zip_archive = kra_archive.zip_archive.borrow_mut();

                for layer in kra_archive.all_layers() {
                    if layer.layer_type()? == KraLayerType::VectorLayer {
                        let content_svg_path = format!(
                            "{}/layers/{}.shapelayer/content.svg",
                            kra_archive.main_doc.image.name, layer.file_name
                        );

                        let content_svg_file = zip_archive.by_name(&content_svg_path)?;
                        let content_svg_data = std::io::read_to_string(content_svg_file)?;

                        if !content_svg_data.is_empty() {
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
        }

        Ok(())
    }
}
