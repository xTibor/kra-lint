use crate::kra_error::KraError;
use crate::kra_params::KraParamsContainer;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct KraPixelizeFilterConfig {
    pub pixel_width: usize,
    pub pixel_height: usize,
}

impl TryFrom<KraParamsContainer> for KraPixelizeFilterConfig {
    type Error = KraError;

    fn try_from(filter_config: KraParamsContainer) -> Result<KraPixelizeFilterConfig, KraError> {
        Ok(KraPixelizeFilterConfig {
            pixel_width: filter_config.get::<usize>("pixelWidth")?,
            pixel_height: filter_config.get::<usize>("pixelHeight")?,
        })
    }
}
