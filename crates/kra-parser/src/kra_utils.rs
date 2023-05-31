use std::collections::VecDeque;
use std::str::FromStr;

use strong_xml::XmlRead;
use ziparchive_ext::ZipArchiveExt;

use crate::kra_archive::KraArchive;
use crate::kra_error::KraError;
use crate::kra_maindoc::{
    KraLayerType, KraMainDocImage, KraMainDocLayer, KraMainDocLayerContainer, KraMainDocMask, KraMaskType,
};
use crate::kra_params::KraParamsContainer;

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

        let svg_path = format!("{}/layers/{}.shapelayer/content.svg", kra_archive.main_doc.image.name, self.file_name);

        zip_archive.read_to_string(&svg_path)?.ok_or(KraError::ContentSvgNotFound { svg_path })
    }

    pub fn color_profile(&self, kra_archive: &KraArchive) -> Result<Vec<u8>, KraError> {
        assert_eq!(self.layer_type, KraLayerType::PaintLayer);

        let mut zip_archive = kra_archive.zip_archive.borrow_mut();

        let color_profile_path = format!(
            "{document_name:}/layers/{layer_name:}.icc",
            document_name = kra_archive.main_doc.image.name,
            layer_name = self.file_name
        );

        zip_archive.read(&color_profile_path)?.ok_or(KraError::ColorProfileNotFound { color_profile_path })
    }

    pub fn filter_config_params(&self, kra_archive: &KraArchive) -> Result<Option<KraParamsContainer>, KraError> {
        assert_eq!(self.layer_type, KraLayerType::FilterLayer);

        let mut zip_archive = kra_archive.zip_archive.borrow_mut();

        let filter_config_path = format!(
            "{document_name:}/layers/{layer_name:}.filterconfig",
            document_name = kra_archive.main_doc.image.name,
            layer_name = self.file_name
        );

        Ok(zip_archive.read_to_string(&filter_config_path)?.as_deref().map(KraParamsContainer::from_str).transpose()?)
    }

    pub fn filter_config<T>(&self, kra_archive: &KraArchive) -> Result<T, KraError>
    where
        T: TryFrom<KraParamsContainer, Error = KraError>,
    {
        let filter_config_params = self.filter_config_params(kra_archive)?.ok_or(KraError::FilterConfigNotFound)?;
        T::try_from(filter_config_params)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl KraMainDocMask {
    pub fn colorize_color_profile(&self, kra_archive: &KraArchive) -> Result<Vec<u8>, KraError> {
        assert_eq!(self.mask_type, KraMaskType::ColorizeMask);

        let mut zip_archive = kra_archive.zip_archive.borrow_mut();

        let color_profile_path = format!(
            "{document_name:}/layers/{mask_name:}.colorizemask/{document_name:}/layers/{mask_name:}.icc",
            document_name = kra_archive.main_doc.image.name,
            mask_name = self.file_name.as_ref().ok_or(KraError::MaskFileNameFieldNotFound)?,
        );

        zip_archive.read(&color_profile_path)?.ok_or(KraError::ColorProfileNotFound { color_profile_path })
    }

    pub fn filter_config_params(&self, kra_archive: &KraArchive) -> Result<Option<KraParamsContainer>, KraError> {
        assert_eq!(self.mask_type, KraMaskType::FilterMask);

        let mut zip_archive = kra_archive.zip_archive.borrow_mut();

        let filter_config_path = format!(
            "{document_name:}/layers/{mask_name:}.filterconfig",
            document_name = kra_archive.main_doc.image.name,
            mask_name = self.file_name.as_ref().ok_or(KraError::MaskFileNameFieldNotFound)?,
        );

        Ok(zip_archive.read_to_string(&filter_config_path)?.as_deref().map(KraParamsContainer::from_str).transpose()?)
    }

    pub fn filter_config<T>(&self, kra_archive: &KraArchive) -> Result<T, KraError>
    where
        T: TryFrom<KraParamsContainer, Error = KraError>,
    {
        let filter_config_params = self.filter_config_params(kra_archive)?.ok_or(KraError::FilterConfigNotFound)?;
        T::try_from(filter_config_params)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl KraMainDocImage {
    pub fn color_profile(&self, kra_archive: &KraArchive) -> Result<Vec<u8>, KraError> {
        let mut zip_archive = kra_archive.zip_archive.borrow_mut();

        let color_profile_path =
            format!("{document_name:}/annotations/icc", document_name = kra_archive.main_doc.image.name);

        zip_archive.read(&color_profile_path)?.ok_or(KraError::ColorProfileNotFound { color_profile_path })
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl KraParamsContainer {
    pub fn get_opt<T: FromStr>(&self, param_name: &str) -> Result<Option<T>, KraError> {
        if let Some(param) = self.into_iter().find(|param| param.name == param_name) {
            let parse_result = param
                .value
                .parse::<T>()
                .map_err(|_| KraError::FailedToParseFilterConfigParam { param_name: param_name.to_owned() });
            Some(parse_result).transpose()
        } else {
            Ok(None)
        }
    }

    pub fn get<T: FromStr>(&self, param_name: &str) -> Result<T, KraError> {
        self.get_opt::<T>(param_name)?
            .ok_or(KraError::CannotFindFilterConfigParam { param_name: param_name.to_owned() })
    }
}
