use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::organization::{Organization, UsageQueryRequest, UsageResponse};

pub struct OrganizationResource {
    pub(crate) client: RunwayClient,
}

impl OrganizationResource {
    pub async fn get(&self) -> Result<Organization, RunwayError> {
        self.client.get("/v1/organization").await
    }

    pub async fn usage(&self, request: UsageQueryRequest) -> Result<UsageResponse, RunwayError> {
        self.client.post("/v1/organization/usage", &request).await
    }
}
