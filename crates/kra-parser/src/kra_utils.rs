use std::collections::VecDeque;

use crate::kra_archive::KraArchive;
use crate::kra_error::KraError;
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(PartialEq)]
pub enum KraLayerType {
    PaintLayer,
    GroupLayer,
    CloneLayer,
    VectorLayer,
    FilterLayer,
    FillLayer,
    FileLayer,
}

impl TryFrom<&str> for KraLayerType {
    type Error = KraError;

    #[rustfmt::skip]
    fn try_from(value: &str) -> Result<KraLayerType, KraError> {
        match value {
            "paintlayer"      => Ok(KraLayerType::PaintLayer ),
            "grouplayer"      => Ok(KraLayerType::GroupLayer ),
            "clonelayer"      => Ok(KraLayerType::CloneLayer ),
            "shapelayer"      => Ok(KraLayerType::VectorLayer),
            "adjustmentlayer" => Ok(KraLayerType::FilterLayer),
            "generatorlayer"  => Ok(KraLayerType::FillLayer  ),
            "filelayer"       => Ok(KraLayerType::FileLayer  ),
            _ => Err(KraError::UnknownLayerNodeType(value.to_string())),
        }
    }
}

impl KraMainDocLayer {
    pub fn layer_type(&self) -> Result<KraLayerType, KraError> {
        self.node_type.as_str().try_into()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(PartialEq)]
pub enum KraMaskType {
    TransparencyMask,
    FilterMask,
    ColorizeMask,
    TransformMask,
    LocalSelection,
}

impl TryFrom<&str> for KraMaskType {
    type Error = KraError;

    #[rustfmt::skip]
    fn try_from(value: &str) -> Result<KraMaskType, KraError> {
        match value {
            "transparencymask" => Ok(KraMaskType::TransparencyMask),
            "filtermask"       => Ok(KraMaskType::FilterMask      ),
            "colorizemask"     => Ok(KraMaskType::ColorizeMask    ),
            "transformmask"    => Ok(KraMaskType::TransformMask   ),
            "selectionmask"    => Ok(KraMaskType::LocalSelection  ),
            _ => Err(KraError::UnknownMaskNodeType(value.to_string())),
        }
    }
}

impl KraMainDocMask {
    pub fn mask_type(&self) -> Result<KraMaskType, KraError> {
        self.node_type.as_str().try_into()
    }
}
