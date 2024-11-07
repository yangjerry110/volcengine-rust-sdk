/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-05 10:39:54
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-05 11:38:06
 * @Description:
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request::RequestVolcengine;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::rds_database;

impl request::ApiRequest for rds_database::CreateDatabaseReq {
    fn to_hashmap(&self) -> HashMap<String, String> {
        request::Request::format_request_to_hashmap(self)
    }

    fn to_body(&self) -> Vec<u8> {
        let result = Vec::new();
        result
    }
}

impl response::ApiResponse for rds_database::CreateDatabaseReq {
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // 解析 JSON 响应体
        let parsed_response: volcengine_sdk_protobuf::protobuf::rds_database::CreateDatabaseReq =
            http_response
                .json()
                .await
                .map_err(|e| error::Error::ErrParseResponse(e))?;

        // set self
        *self = parsed_response;

        // return
        Ok(())
    }
}
