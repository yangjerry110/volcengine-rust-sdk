use crate::volcengine::error::error;
use crate::volcengine::request::operation_config;
use crate::volcengine::request::request;
use crate::volcengine::request::sign::{self, SignRequest};
use chrono::Utc;
use hex::encode;
use reqwest::{Client, RequestBuilder};
use std::collections::HashMap;
use std::future::Future;

pub trait SendRequest {
    /// 设置请求
    fn set_request(request: &request::Request) -> Self;

    /// 发送请求
    fn send<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> impl Future<Output = Result<reqwest::Response, error::Error>>;

    /// 构建请求头
    fn build_request_headers<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request_builder: RequestBuilder,
        request: &T,
    ) -> Result<RequestBuilder, error::Error>;

    /// 构建授权请求头
    fn build_request_headers_authorization<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request: &T,
        request_builder: RequestBuilder,
    ) -> Result<RequestBuilder, error::Error>;

    /// 构建请求
    fn build_request<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> Result<RequestBuilder, error::Error>;

    /// 格式化请求为HashMap
    fn format_request_hashmap<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> HashMap<String, String>;

    /// 获取当前时间的格式化字符串
    fn get_x_date(&self) -> String;
}

/// 发送请求的实现
#[derive(Debug, Clone)]
pub struct Send {
    request: request::Request,
}

impl SendRequest for Send {
    /// 设置请求的实现
    ///
    /// # 参数
    /// - `request` : 传入的请求对象
    ///
    /// # 返回
    /// 返回 `Send` 实例
    fn set_request(request: &request::Request) -> Self {
        Send {
            request: request.clone(),
        }
    }

    /// 发送请求
    ///
    /// # 参数
    /// - `request` : 要发送的请求对象
    ///
    /// # 返回
    /// 返回 `Result` 包含 `reqwest::Response` 或错误
    async fn send<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> Result<reqwest::Response, error::Error> {
        // 获取当前时间
        let now_date_string = self.get_x_date();
        let now_date = now_date_string.as_str();

        // 构建请求
        let request_builder = self.build_request(request)?;

        // 构建请求头
        let request_builder = self.build_request_headers(&now_date, request_builder, request)?;

        // 添加授权请求头
        let request_builder =
            self.build_request_headers_authorization(&now_date, request, request_builder)?;

        // 发送请求并处理响应
        let response = request_builder
            .send()
            .await
            .map_err(|e| error::Error::ErrRequest(e))?;

        Ok(response)
    }

    /// 构建请求头
    ///
    /// # 参数
    /// - `now_date` : 当前时间字符串
    /// - `request_builder` : 请求构建器
    /// - `request` : 请求对象
    ///
    /// # 返回
    /// 返回构建后的请求构建器或错误
    fn build_request_headers<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request_builder: RequestBuilder,
        request: &T,
    ) -> Result<RequestBuilder, error::Error> {
        // 签名对象
        let request_sign = sign::Sign::default();

        // 克隆请求构建器
        let request_builder_clone = request_builder
            .try_clone()
            .ok_or_else(|| error::Error::ErrRequestBuilderIsNone)?;

        // 添加基础头部信息
        let request_builder_with_headers = request_builder_clone.header("X-Date", now_date);

        // 获取请求方法并根据方法设置不同的请求头
        let reqwest_request = request_builder
            .build()
            .map_err(|e| error::Error::ErrRequest(e))?;

        let method = reqwest_request.method().as_str();
        if method == "POST" {
            // 对请求体进行哈希处理，生成摘要
            let payload_hash = encode(request_sign.hash_sha256(request.to_body().as_ref()));

            // 添加 `X-Content-Sha256` 头
            let request_builder_with_headers =
                request_builder_with_headers.header("X-Content-Sha256", payload_hash);
            return Ok(request_builder_with_headers);
        }

        Ok(request_builder_with_headers)
    }

    /// 构建授权请求头
    ///
    /// # 参数
    /// - `now_date` : 当前时间字符串
    /// - `request` : 请求对象
    /// - `request_builder` : 请求构建器
    ///
    /// # 返回
    /// 返回带有授权信息的请求构建器
    fn build_request_headers_authorization<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request: &T,
        request_builder: RequestBuilder,
    ) -> Result<RequestBuilder, error::Error> {
        // 克隆请求构建器
        let request_builder_clone = request_builder
            .try_clone()
            .ok_or_else(|| error::Error::ErrRequestBuilderIsNone)?;

        // 构建请求并获取签名头部信息
        let reqwest_request = request_builder_clone
            .build()
            .map_err(|e| error::Error::ErrRequest(e))?;
        let request_sign = sign::Sign::default();
        let sign_headers = request_sign.get_sign_header_keys(&reqwest_request);

        let mut authorization_sign_headers: Vec<String> = Vec::new();
        for sign_header in sign_headers {
            authorization_sign_headers.push(sign_header.to_lowercase());
        }
        let authorization_sign_header_str = authorization_sign_headers.join(";");

        // 生成签名
        let signature =
            request_sign.build_signature(now_date, &self.request, request, &request_builder)?;

        // 构建 `Authorization` 头部
        let short_date = &now_date[..8];
        let authorization = format!(
            r#"HMAC-SHA256 Credential={AccessKey}/{ShortDate}/{Region}/{Service}/request, SignedHeaders={SignedHeaders}, Signature={Signature}"#,
            AccessKey = self.request.config.config.credentials.access_key_id,
            ShortDate = short_date,
            Region = self.request.client_info.signing_region,
            Service = self.request.client_info.service_name.as_str(),
            SignedHeaders = authorization_sign_header_str,
            Signature = signature,
        );

        // 添加授权头到请求中
        let request_builder_with_headers = request_builder.header("Authorization", authorization);

        Ok(request_builder_with_headers)
    }

    /// 构建请求
    ///
    /// # 参数
    /// - `request` : 请求对象
    ///
    /// # 返回
    /// 返回构建器或错误
    fn build_request<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> Result<RequestBuilder, error::Error> {
        let client = Client::new();

        // 格式化请求参数为查询字符串
        let query_string = self
            .format_request_hashmap(request)
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        let request_addr = format!(
            "{}{}?{}",
            self.request.config.endpoint,
            self.request.operation.clone().http_path.to_string(),
            query_string
        );

        // 根据 HTTP 方法构建不同的请求
        let request_builder = match self.request.operation.http_method {
            operation_config::operation_http_method::OperationHttpMethod::GET => {
                client.get(&request_addr)
            }
            operation_config::operation_http_method::OperationHttpMethod::POST => {
                let body = request.to_body();
                client.post(&request_addr).body(body)
            }
        };

        Ok(request_builder)
    }

    /// 格式化请求为 HashMap
    ///
    /// # 参数
    /// - `request` : 请求对象
    ///
    /// # 返回
    /// 返回格式化后的 HashMap
    fn format_request_hashmap<T: request::ApiRequest>(
        &self,
        request: &T,
    ) -> HashMap<String, String> {
        let mut request_hashmap = request.to_hashmap();

        request_hashmap.insert(
            String::from("Action"),
            self.request.operation.name.to_string(),
        );
        request_hashmap.insert(
            String::from("Version"),
            self.request.client_info.api_version.to_string(),
        );

        request_hashmap
    }

    /// 获取当前时间的格式化字符串 (UTC)
    fn get_x_date(&self) -> String {
        Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
    }
}
