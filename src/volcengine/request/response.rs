/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-21 18:19:47
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-23 18:36:35
 * @Description: response
 */
use crate::volcengine::error::error;
use serde::{Deserialize, Serialize};
use std::future::Future;

pub trait ApiResponse {
    fn to_struct(
        &mut self,
        http_response: reqwest::Response,
    ) -> impl Future<Output = Result<(), error::Error>>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApiResponseMetadata {
    #[serde(rename = "RequestId")]
    pub request_id: String,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Service")]
    pub service: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "Error")]
    pub error: Option<ApiResponseMetadataErrData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApiResponseMetadataErrData {
    #[serde(rename = "CodeN")]
    pub code_no: Option<i64>,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
}
