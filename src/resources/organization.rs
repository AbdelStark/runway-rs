use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::organization::{Organization, UsageQueryRequest, UsageResponse};

pub struct OrganizationResource {
    pub(crate) client: RunwayClient,
}

impl OrganizationResource {
    pub async fn retrieve(&self) -> Result<Organization, RunwayError> {
        Ok(self
            .retrieve_with_options(RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<WithResponse<Organization>, RunwayError> {
        self.client
            .get_with_options("/v1/organization", &options)
            .await
    }

    pub async fn retrieve_usage(
        &self,
        request: UsageQueryRequest,
    ) -> Result<UsageResponse, RunwayError> {
        Ok(self
            .retrieve_usage_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_usage_with_options(
        &self,
        request: UsageQueryRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<UsageResponse>, RunwayError> {
        self.client
            .post_with_options("/v1/organization/usage", &request, &options)
            .await
    }

    pub async fn get(&self) -> Result<Organization, RunwayError> {
        self.retrieve().await
    }

    pub async fn usage(&self, request: UsageQueryRequest) -> Result<UsageResponse, RunwayError> {
        self.retrieve_usage(request).await
    }
}
