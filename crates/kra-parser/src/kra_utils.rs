use std::collections::VecDeque;

use crate::kra_archive::KraArchive;
use crate::kra_error::KraError;
use crate::kra_maindoc::{KraLayerType, KraMainDocLayer, KraMainDocLayerContainer, KraMainDocMask, KraMaskType};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct KraRecursiveLayerIterator<'a> {
    queue: VecDeque<&'a KraMainDocLayer>,
}

impl<'a> Iterator for KraRecursiveLayerIterator<'a> {
    type Item = &'a KraMainDocLayer;

    fn next(&mut self) -> Option<Self::Item> {
        let layer = self.queue.pop_front()?;

        if let Some(layer_container) = layer.layer_container.as_ref() {
            self.queue.extend(layer_container.0.iter());
            self.queue.rotate_right(layer_container.0.len());
        }

        Some(layer)
    }
}

impl KraMainDocLayerContainer {
    pub fn iter_recursive(&self) -> impl Iterator<Item = &KraMainDocLayer> {
        KraRecursiveLayerIterator { queue: self.0.iter().collect() }
    }
}

impl KraMainDocLayer {
    #[allow(dead_code)]
    pub fn iter_recursive(&self) -> impl Iterator<Item = &KraMainDocLayer> {
        KraRecursiveLayerIterator { queue: [self].into() }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl KraArchive {
    pub fn all_layers(&self) -> impl Iterator<Item = &KraMainDocLayer> {
        self.main_doc.image.layer_container.iter_recursive()
    }

    pub fn all_layers_by_type(&self, layer_type: KraLayerType) -> impl Iterator<Item = &KraMainDocLayer> {
        self.all_layers().filter(move |kra_layer| kra_layer.layer_type == layer_type)
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
                    .map(|mask_container| mask_container.into_iter().map(move |mask| (layer, mask)))
            })
            .flatten()
    }

    pub fn all_masks_by_type(
        &self,
        mask_type: KraMaskType,
    ) -> impl Iterator<Item = (&KraMainDocLayer, &KraMainDocMask)> {
        self.all_masks().filter(move |(_, kra_mask)| kra_mask.mask_type == mask_type)
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
