/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 10:50:53
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 10:54:01
 * @Description: operation http path
 */

// Enum representing possible HTTP paths, currently only `Default` ("/")
#[derive(Debug, Clone)]
pub enum OperationHttpPath {
    Default,
}

/**
 * @description: impl ToString for OperationHttpPath
 * @author: Jerry.Yang
 * @date: 2024-10-30 10:54:22
 * @return {*}
 */
impl ToString for OperationHttpPath {
    fn to_string(&self) -> String {
        match self {
            OperationHttpPath::Default => "/",
        }
        .to_string()
    }
}
