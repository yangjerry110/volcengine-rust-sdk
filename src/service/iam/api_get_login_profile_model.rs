/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-25 15:10:33
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-28 11:08:07
 * @Description: get login profile model
 */
use crate::volcengine::error::error;
use crate::volcengine::request::{request, response};
use std::collections::HashMap;
use volcengine_sdk_protobuf::protobuf::iam_user;

impl request::ApiRequest for iam_user::GetLoginProfileReq {
    fn to_hashmap(&self) -> HashMap<String, String> {
        // 将结构体序列化为 JSON Value
        let value = serde_json::to_value(self).unwrap();
        // 转换为 Map<String, Value>
        let map = value.as_object().unwrap();

        // 将每个字段从 Value 转换为 String，并过滤掉空值
        map.iter()
            .filter_map(|(k, v)| {
                if v.is_null() || v == "" || v == "null" {
                    None // 跳过空值
                } else {
                    Some((k.clone(), v.to_string().replace("\"", ""))) // 移除引号并保留非空值
                }
            })
            .collect()
    }

    fn to_body(&self) -> Vec<u8> {
        let result = Vec::new();
        result
    }
}

impl response::ApiResponse for iam_user::GetLoginProfileResp {
    async fn to_struct(&mut self, http_response: reqwest::Response) -> Result<(), error::Error> {
        // 解析 JSON 响应体
        let parsed_response: volcengine_sdk_protobuf::protobuf::iam_user::GetLoginProfileResp =
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
