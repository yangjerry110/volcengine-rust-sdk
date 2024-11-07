/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 15:07:52
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-28 16:33:44
 * @Description: 定义 `Client` 和其构建器 `ClientBuilder`，用于封装客户端配置信息、客户端信息以及请求处理器
 */

use super::client_info;
use crate::volcengine::client::config;
use crate::volcengine::error::error;
use crate::volcengine::request::handles;

// 定义 `Client` 结构体，包含客户端信息、配置信息以及请求处理器
#[derive(Debug, Clone)]
pub struct Client {
    pub client_info: client_info::ClientInfo, // 客户端信息
    pub config: config::Config,               // 客户端配置
    pub handles: handles::Handles,            // 请求处理器
}

// `Client` 的实现，提供了一个 `builder` 函数，用于创建 `ClientBuilder`
impl Client {
    // 创建 `ClientBuilder`，用于逐步构建 `Client`
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            client_info: None,
            config: None,
            handles: None,
        }
    }
}

// 定义 `ClientBuilder` 结构体，用于构建 `Client`
pub struct ClientBuilder {
    pub client_info: Option<client_info::ClientInfo>, // 可选的客户端信息
    pub config: Option<config::Config>,               // 可选的客户端配置
    pub handles: Option<handles::Handles>,            // 可选的请求处理器
}

// `ClientBuilder` 的实现，提供了链式调用的方法来设置 `Client` 的各个字段，并最终构建 `Client`
impl ClientBuilder {
    // 设置 `client_info` 字段
    pub fn with_client_info(mut self, client_info: &client_info::ClientInfo) -> Self {
        self.client_info = Some(client_info.clone());
        self
    }

    // 设置 `config` 字段
    pub fn with_config(mut self, config: &config::Config) -> Self {
        self.config = Some(config.clone());
        self
    }

    // 设置 `handles` 字段
    pub fn with_handles(mut self, handles: &handles::Handles) -> Self {
        self.handles = Some(handles.clone());
        self
    }

    // 构建 `Client`，并进行必要的字段验证
    pub fn build(self) -> Result<Client, error::Error> {
        // 验证 `client_info` 字段是否已设置
        if self.client_info.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientNo(
                "client_info".to_string(),
            ));
        }

        // 验证 `config` 字段是否已设置
        if self.config.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientNo(
                "config".to_string(),
            ));
        }

        // 验证 `handles` 字段是否已设置
        if self.handles.is_none() {
            return Err(error::Error::ErrUtilClientBuildClientNo(
                "handles".to_string(),
            ));
        }

        // 返回构建好的 `Client` 实例
        Ok(Client {
            client_info: self.client_info.unwrap(),
            config: self.config.unwrap(),
            handles: self.handles.unwrap(),
        })
    }
}
