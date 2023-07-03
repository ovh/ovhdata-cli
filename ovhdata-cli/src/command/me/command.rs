use ovhdata_common::ovhapi::{AuthApi, OVHapiV6Client};

use crate::utils::ui::printer::{Output, Printer};
use crate::utils::Result;

pub struct MeCommand {
    rcp_client: OVHapiV6Client,
}

impl MeCommand {
    pub fn new(rcp_client: OVHapiV6Client) -> Self {
        Self { rcp_client }
    }

    /// Get user related information
    pub async fn me(&self, output: Output) -> Result<()> {
        let me = self.rcp_client.me().await?;

        Printer::print_object(&me, &output)?;

        Ok(())
    }
}
