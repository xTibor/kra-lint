use std::collections::VecDeque;

use crate::kra_archive::KraArchive;
use crate::kra_error::KraError;
use crate::kra_maindoc::{KraLayerType, KraMainDocLayer, KraMainDocLayerContainer, KraMainDocMask};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct KraLayerIterator<'a> {
    queue: VecDeque<&'a KraMainDocLayer>,
}

impl<'a> Iterator for KraLayerIterator<'a> {
    type Item = &'a KraMainDocLayer;

    fn next(&mut self) -> Option<Self::Item> {
        let layer = self.queue.pop_front()?;

        if let Some(layer_container) = layer.layer_container.as_ref() {
            self.queue.extend(layer_container.layers.iter());
            self.queue.rotate_right(layer_container.layers.len());
        }

        Some(layer)
    }
}

impl KraMainDocLayerContainer {
    pub fn iter_recursive(&self) -> impl Iterator<Item = &KraMainDocLayer> {
        KraLayerIterator { queue: self.layers.iter().collect() }
    }
}

impl KraMainDocLayer {
    #[allow(dead_code)]
    pub fn iter_recursive(&self) -> impl Iterator<Item = &KraMainDocLayer> {
        KraLayerIterator { queue: [self].into() }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl KraArchive {
    pub fn all_layers(&self) -> impl Iterator<Item = &KraMainDocLayer> {
        self.main_doc.image.layer_container.iter_recursive()
    }

    pub fn all_masks(&self) -> impl Iterator<Item = (&KraMainDocLayer, &KraMainDocMask)> {
        self.main_doc
            .image
            .layer_container
            .iter_recursive()
            .filter_map(|layer| {
                layer
                    .mask_container
                    .as_ref()
                    .map(|mask_container| mask_container.masks.iter().map(move |mask| (layer, mask)))
            })
            .flatten()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl KraMainDocLayer {
    pub fn content_svg(&self, kra_archive: &KraArchive) -> Result<String, KraError> {
        assert_eq!(self.layer_type, KraLayerType::VectorLayer);

        let mut zip_archive = kra_archive.zip_archive.borrow_mut();

        let content_svg_file = zip_archive.by_name(&format!(
            "{}/layers/{}.shapelayer/content.svg",
            kra_archive.main_doc.image.name, self.file_name
        ))?;

        Ok(std::io::read_to_string(content_svg_file)?)
    }
}
