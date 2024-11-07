/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 14:58:16
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-28 10:40:37
 * @Description: sign
 */
use crate::volcengine::error::error;
use crate::volcengine::request::request;
use hex::encode;
use hmac::{Hmac, Mac};
use reqwest::RequestBuilder;
use sha2::{Digest, Sha256};
use url::Url;

pub trait SignRequest {
    fn hmac_sha256(&self, key: &[u8], content: &str) -> Vec<u8>;
    fn get_signed_key(&self, secret_key: &str, date: &str, region: &str, service: &str) -> Vec<u8>;
    fn hash_sha256(&self, data: &[u8]) -> Vec<u8>;
    fn get_sign_header_keys(&self, reqwest_request: &reqwest::Request) -> Vec<&str>;
    fn build_signature<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request: &request::Request,
        api_request: &T,
        request_builder: &RequestBuilder,
    ) -> Result<String, error::Error>;
    fn build_sign_request_query_str<T: request::ApiRequest>(
        &self,
        request: &request::Request,
        api_request: &T,
    ) -> Result<String, error::Error>;
    fn build_sign_headers_str(
        &self,
        reqwest_request: &reqwest::Request,
    ) -> Result<String, error::Error>;
    fn build_sign_header_keys_str(
        &self,
        reqwest_request: &reqwest::Request,
    ) -> Result<String, error::Error>;
    fn build_sign_payload<T: request::ApiRequest>(
        &self,
        api_request: &T,
    ) -> Result<String, error::Error>;
}

#[derive(Debug, Default, Clone)]
pub struct Sign;

/// SignClient for Sign
impl SignRequest for Sign {
    /// build_signature
    ///
    /// # 参数
    /// - `&self`
    /// - `date` : &str
    /// - `request` : &request::Request
    /// - `api_request` : &Box<dyn request::ApiRequest>
    /// - `request_builder` : &RequestBuilder
    ///
    /// # 返回
    /// 成功返回 String, 错误返回 error::Error
    fn build_signature<T: request::ApiRequest>(
        &self,
        now_date: &str,
        request: &request::Request,
        api_request: &T,
        request_builder: &RequestBuilder,
    ) -> Result<String, error::Error> {
        // clone request_builder
        let request_builder_clone = request_builder
            .try_clone()
            .ok_or_else(|| error::Error::ErrRequestBuilderIsNone)?;

        // build request_builder
        let reqwest_request = request_builder_clone
            .build()
            .map_err(|e| error::Error::ErrRequest(e))?;

        // define method
        let method = reqwest_request.method().as_str();
        // define url
        // let url = reqwest_request.url().as_str();

        // get request_query_str
        let api_request_query_str = self.build_sign_request_query_str(request, api_request)?;

        // get sign headers_str
        let sign_headers_str = self.build_sign_headers_str(&reqwest_request)?;

        // get sign header_keys_str
        let sign_header_keys_str = self.build_sign_header_keys_str(&reqwest_request)?;

        // get sign payload
        let sign_payload = self.build_sign_payload(api_request)?;

        // build CanonicalRequest
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,
            "/",
            api_request_query_str,
            sign_headers_str,
            sign_header_keys_str,
            sign_payload
        );

        // build hash canonical_request
        let hash_canonical_request = hex::encode(self.hash_sha256(canonical_request.as_bytes()));

        // define CredentialScope
        let sign_credential_scope_date = &now_date[..8];
        let sign_credential_scope = format!(
            "{}/{}/{}/request",
            sign_credential_scope_date,
            request.client_info.signing_region,
            request.client_info.service_name.as_str()
        );

        // define StringToSign
        let sign_string = format!(
            "HMAC-SHA256\n{}\n{}\n{}",
            now_date, sign_credential_scope, hash_canonical_request
        );

        // define sign_key_date
        let sign_key_date_str = &now_date[..8];

        // define signed_key
        let signed_key = self.get_signed_key(
            request.config.config.credentials.secret_access_key.as_str(),
            sign_key_date_str,
            request.client_info.signing_region.as_str(),
            request.client_info.service_name.as_str(),
        );

        // define signature
        let signature = encode(self.hmac_sha256(&signed_key, &sign_string));

        // return
        Ok(signature)
    }

    /// build_sign_payload
    ///
    /// # 参数
    /// - `&self`
    /// - `api_request` : &Box<dyn request::ApiRequest>
    ///
    /// # 返回
    /// 成功返回 String, 错误返回 error::Error
    fn build_sign_payload<T: request::ApiRequest>(
        &self,
        api_request: &T,
    ) -> Result<String, error::Error> {
        // define payload_hash
        let payload_hash = encode(self.hash_sha256(api_request.to_body().as_ref()));

        // return
        Ok(payload_hash)
    }

    /// build_sign_header_keys_str
    ///
    /// # 参数
    /// - `&self`
    ///
    /// # 返回
    /// 成功返回 String, 错误返回 error::Error
    fn build_sign_header_keys_str(
        &self,
        reqwest_request: &reqwest::Request,
    ) -> Result<String, error::Error> {
        // define sign_header_keys
        let sign_header_keys = self.get_sign_header_keys(reqwest_request);

        // define lower_sign_header_keys
        let mut lower_sign_header_keys: Vec<String> = Vec::new();

        // for sign_header_keys
        for sign_header_key in sign_header_keys {
            let sign_header_key_string = sign_header_key.to_string();
            lower_sign_header_keys.push(sign_header_key_string.to_lowercase());
        }

        // return
        Ok(lower_sign_header_keys.join(";"))
    }

    /// build_sign_headers_str
    ///
    /// # 参数
    /// - `&self`
    /// - `reqwest_request` : &reqwest::Request
    ///
    /// # 返回
    /// 成功返回 String, 错误返回 error::Error
    fn build_sign_headers_str(
        &self,
        reqwest_request: &reqwest::Request,
    ) -> Result<String, error::Error> {
        //define headers_str
        let mut sign_headers: Vec<String> = Vec::new();

        // define sign_header_keys
        let sign_header_keys = self.get_sign_header_keys(reqwest_request);

        // for sign_header_keys
        for sign_header_key in sign_header_keys {
            // 判断当前sign_header_key 是否为Host
            if sign_header_key == "Host" {
                // get host
                let url = reqwest_request.url().as_str();
                let parse_url =
                    Url::parse(url).map_err(|e| error::Error::ErrRequestSignGetHost(e))?;

                // 判断是否存在
                if parse_url.host_str().is_none() {
                    return Err(error::Error::ErrRequestSignGetHostNone);
                }

                // get host_str
                let host_str = parse_url.host_str().unwrap();

                // to lower sign_header_key
                let lower_sign_header_key = sign_header_key.to_lowercase();
                sign_headers.push(format!("{}:{}", lower_sign_header_key, host_str));
                continue;
            }

            // 其他的，都从request的headers头里面获取
            let header_value = reqwest_request.headers().get(sign_header_key);

            // 判断是否不存在
            if header_value.is_none() {
                return Err(error::Error::ErrRequestSignGetHeaderNone(
                    sign_header_key.to_string(),
                ));
            }

            // reset header_value
            let header_value = header_value.unwrap();

            // get header_value str
            let header_value_str = header_value
                .to_str()
                .map_err(|e| error::Error::ErrRequestHeaderIsErr(e))?;

            // to lower sign_header_key
            let lower_sign_header_key = sign_header_key.to_lowercase();
            sign_headers.push(format!("{}:{}", lower_sign_header_key, header_value_str));
        }

        // 假如是GET的请求，增加一个空
        let method = reqwest_request.method().as_str();
        if method == "GET" {
            sign_headers.push(String::from(""));
        }

        // join sign_headers
        let sign_headers_str = sign_headers.join("\n");

        // return
        Ok(sign_headers_str)
    }

    /// build_sign_request_query_str
    ///
    /// # 参数
    /// - `&self`
    /// - `now_date` : DateTime<Local>
    /// - `request` : &request::Request
    /// - `api_request` : &Box<dyn request::ApiRequest>
    ///
    /// # 返回
    /// 成功返回 String, 错误返回 error::Error
    fn build_sign_request_query_str<T: request::ApiRequest>(
        &self,
        request: &request::Request,
        api_request: &T,
    ) -> Result<String, error::Error> {
        // define request_hashmap
        let mut request_hashmap = api_request.to_hashmap();

        // insert Action Version
        request_hashmap.insert(
            String::from("Action"),
            request.operation.clone().name.to_string(),
        );
        request_hashmap.insert(
            String::from("Version"),
            request.client_info.api_version.to_string(),
        );

        // 获取 HashMap 的键并转换为 Vec
        let mut keys: Vec<String> = request_hashmap.keys().cloned().collect();

        // 对键进行排序
        keys.sort();

        // for request_hashmap
        let mut request_query_str = String::from("");

        // for keys
        for key in keys {
            // get request_value
            let request_value = request_hashmap.get(&key);

            // if none
            if request_value.is_none() {
                continue;
            }

            // reset request_value
            let request_value = request_value.unwrap();

            // set request_query_str
            request_query_str.push_str(&format!("{}={}&", key, request_value));
        }

        // 去掉最后一个 '&' 符号
        if !request_query_str.is_empty() {
            request_query_str.pop();
        }

        // return
        Ok(request_query_str)
    }

    /// get_sign_header_keys
    ///
    /// # 参数
    /// - `&self`
    /// - `reqwest_request` : &reqwest::Request
    ///
    /// # 返回
    /// 成功返回 Vec<&str>
    fn get_sign_header_keys(&self, reqwest_request: &reqwest::Request) -> Vec<&str> {
        // get request method
        let method = reqwest_request.method().as_str();

        if method == "POST" {
            let result = vec!["Host", "X-Date", "X-Content-Sha256"];
            return result;
        }

        // define sign_header_keys
        vec!["Host", "X-Date"]
    }

    /// hmac_sha256
    ///
    /// # 参数
    /// - `&self`
    /// - `key` : &[u8]
    /// - `content` : &str
    ///
    /// # 返回
    /// 返回 Vec<u8>
    fn hmac_sha256(&self, key: &[u8], content: &str) -> Vec<u8> {
        let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
        mac.update(content.as_bytes());
        mac.finalize().into_bytes().to_vec()
    }

    /// get_signed_key
    ///
    /// # 参数
    /// - `&self`
    /// - `secret_key` : &str
    /// - `date` : &str
    /// - `region` : &str
    /// - `service` : &str
    ///
    /// # 返回
    /// Vec<u8>
    fn get_signed_key(&self, secret_key: &str, date: &str, region: &str, service: &str) -> Vec<u8> {
        let k_date = self.hmac_sha256(secret_key.as_bytes(), date);
        let k_region = self.hmac_sha256(&k_date, region);
        let k_service = self.hmac_sha256(&k_region, service);
        self.hmac_sha256(&k_service, "request")
    }

    /// hash_sha256
    ///
    /// # 参数
    /// - `&self`
    /// - `date` : &[u8]
    ///
    /// # 返回
    /// Vec<u8>
    fn hash_sha256(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
