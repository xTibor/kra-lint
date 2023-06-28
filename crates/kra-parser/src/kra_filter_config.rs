use crate::kra_error::KraError;
use crate::kra_filter_params::KraFilterParamsContainer;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct KraPixelizeFilterConfig {
    pub pixel_width: usize,
    pub pixel_height: usize,
}

impl TryFrom<KraFilterParamsContainer> for KraPixelizeFilterConfig {
    type Error = KraError;

    fn try_from(filter_params: KraFilterParamsContainer) -> Result<KraPixelizeFilterConfig, KraError> {
        Ok(KraPixelizeFilterConfig {
            pixel_width: filter_params.get::<usize>("pixelWidth")?,
            pixel_height: filter_params.get::<usize>("pixelHeight")?,
        })
    }
}
