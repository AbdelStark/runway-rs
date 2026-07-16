//! Client for Runway's production recipe workflows.

use serde::Serialize;

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::recipe::{
    RecipeAdLocalizationRequest, RecipeMarketingStockImageRequest, RecipeMultiShotVideoRequest,
    RecipeProductAdRequest, RecipeProductCampaignImageRequest, RecipeProductSwapRequest,
    RecipeProductUgcRequest,
};
use crate::types::task::TaskCreateResponse;

/// Operations for launching Runway recipe workflows.
pub struct RecipesResource {
    pub(crate) client: RunwayClient,
}

impl RecipesResource {
    async fn submit<T: Serialize>(
        &self,
        path: &str,
        request: &T,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options(path, request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.continuation_client(&options)?, response.data.id),
            response: response.response,
        })
    }

    /// Localize an advertisement image for a target language.
    pub async fn ad_localization(
        &self,
        request: RecipeAdLocalizationRequest,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .ad_localization_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Localize an advertisement with per-request transport overrides.
    pub async fn ad_localization_with_options(
        &self,
        request: RecipeAdLocalizationRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        self.submit("/v1/recipes/ad_localization", &request, options)
            .await
    }

    /// Generate a marketing stock image from a creative brief.
    pub async fn marketing_stock_image(
        &self,
        request: RecipeMarketingStockImageRequest,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .marketing_stock_image_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Generate a marketing stock image with per-request transport overrides.
    pub async fn marketing_stock_image_with_options(
        &self,
        request: RecipeMarketingStockImageRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        self.submit("/v1/recipes/marketing_stock_image", &request, options)
            .await
    }

    /// Generate a multi-cut video from an automatic or custom shot plan.
    pub async fn multi_shot_video(
        &self,
        request: RecipeMultiShotVideoRequest,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .multi_shot_video_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Generate a multi-shot video with per-request transport overrides.
    pub async fn multi_shot_video_with_options(
        &self,
        request: RecipeMultiShotVideoRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        self.submit("/v1/recipes/multi_shot_video", &request, options)
            .await
    }

    /// Generate a cinematic product advertisement.
    pub async fn product_ad(
        &self,
        request: RecipeProductAdRequest,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .product_ad_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Generate a product advertisement with per-request transport overrides.
    pub async fn product_ad_with_options(
        &self,
        request: RecipeProductAdRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        self.submit("/v1/recipes/product_ad", &request, options)
            .await
    }

    /// Generate a fashion product campaign image set.
    pub async fn product_campaign_image(
        &self,
        request: RecipeProductCampaignImageRequest,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .product_campaign_image_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Generate campaign images with per-request transport overrides.
    pub async fn product_campaign_image_with_options(
        &self,
        request: RecipeProductCampaignImageRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        self.submit("/v1/recipes/product_campaign_image", &request, options)
            .await
    }

    /// Replace a product in a reference video.
    pub async fn product_swap(
        &self,
        request: RecipeProductSwapRequest,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .product_swap_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Replace a product with per-request transport overrides.
    pub async fn product_swap_with_options(
        &self,
        request: RecipeProductSwapRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        self.submit("/v1/recipes/product_swap", &request, options)
            .await
    }

    /// Generate a vertical user-generated-content product advertisement.
    pub async fn product_ugc(
        &self,
        request: RecipeProductUgcRequest,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .product_ugc_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Generate a product-UGC video with per-request transport overrides.
    pub async fn product_ugc_with_options(
        &self,
        request: RecipeProductUgcRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        self.submit("/v1/recipes/product_ugc", &request, options)
            .await
    }
}
