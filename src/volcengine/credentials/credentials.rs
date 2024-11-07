/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-17 10:34:24
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-28 11:21:08
 * @Description: credentials
 */
#[derive(Debug, Clone)]
pub struct Credentials {
    pub access_key_id: String,
    pub secret_access_key: String,
}

impl Credentials {
    /// new
    ///
    /// # 参数
    /// - `access_key_id` : &str
    /// - `secret_access_key` : &str
    ///
    /// # 返回
    /// 成功返回 self
    pub fn new(access_key_id: &str, secret_access_key: &str) -> Self {
        Credentials {
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string(),
        }
    }
}
