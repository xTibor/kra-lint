use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KraLayerType {
    PaintLayer,
    GroupLayer,
    CloneLayer,
    VectorLayer,
    FilterLayer,
    FillLayer,
    FileLayer,
}

impl KraMainDocLayer {
    #[rustfmt::skip]
    pub fn layer_type(&self) -> Result<KraLayerType, KraError> {
        match self.node_type.as_str() {
            "paintlayer"      => Ok(KraLayerType::PaintLayer ),
            "grouplayer"      => Ok(KraLayerType::GroupLayer ),
            "clonelayer"      => Ok(KraLayerType::CloneLayer ),
            "shapelayer"      => Ok(KraLayerType::VectorLayer),
            "adjustmentlayer" => Ok(KraLayerType::FilterLayer),
            "generatorlayer"  => Ok(KraLayerType::FillLayer  ),
            "filelayer"       => Ok(KraLayerType::FileLayer  ),
            _ => Err(KraError::UnknownLayerNodeType(self.node_type.clone())),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KraMaskType {
    TransparencyMask,
    FilterMask,
    ColorizeMask,
    TransformMask,
    LocalSelection,
}

impl KraMainDocMask {
    #[rustfmt::skip]
    pub fn mask_type(&self) -> Result<KraMaskType, KraError> {
        match self.node_type.as_str() {
            "transparencymask" => Ok(KraMaskType::TransparencyMask),
            "filtermask"       => Ok(KraMaskType::FilterMask      ),
            "colorizemask"     => Ok(KraMaskType::ColorizeMask    ),
            "transformmask"    => Ok(KraMaskType::TransformMask   ),
            "selectionmask"    => Ok(KraMaskType::LocalSelection  ),
            _ => Err(KraError::UnknownMaskNodeType(self.node_type.clone())),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl KraMainDocLayer {
    pub fn content_svg(&self, kra_archive: &KraArchive) -> Result<String, KraError> {
        assert_eq!(self.layer_type()?, KraLayerType::VectorLayer);

        let mut zip_archive = kra_archive.zip_archive.borrow_mut();

        let content_svg_file = zip_archive.by_name(&format!(
            "{}/layers/{}.shapelayer/content.svg",
            kra_archive.main_doc.image.name, self.file_name
        ))?;

        Ok(std::io::read_to_string(content_svg_file)?)
    }
}
