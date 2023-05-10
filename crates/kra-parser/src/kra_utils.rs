use std::collections::VecDeque;

use crate::kra_archive::KraArchive;
use crate::kra_maindoc::{KraMainDocLayer, KraMainDocLayerContainer, KraMainDocMask};

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
