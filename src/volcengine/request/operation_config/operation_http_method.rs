/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 10:50:35
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-30 10:52:23
 * @Description: operation http method
 */

// Enum representing possible HTTP methods, such as GET and POST
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum OperationHttpMethod {
    GET,
    POST,
}
