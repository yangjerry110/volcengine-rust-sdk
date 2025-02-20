/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 10:50:35
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:35:29
 * @Description: operation http method
 */

/// Enum representing possible HTTP methods that can be used in API operations.
/// This enum provides a type - safe way to specify the HTTP method for an API request.
/// It currently includes the two most commonly used HTTP methods: GET and POST.
/// The `Debug` derive allows for easy debugging by providing a default implementation
/// of the `fmt::Debug` trait, which enables printing the enum variants in a readable format.
/// The `Clone` derive allows for creating copies of the enum values when needed.
#[derive(Debug, Clone)]
pub enum OperationHttpMethod {
    /// Represents the HTTP GET method.
    /// The GET method is used to request data from a specified resource.
    /// It is a safe and idempotent method, meaning that multiple identical requests
    /// should have the same effect as a single request.
    GET,
    /// Represents the HTTP POST method.
    /// The POST method is used to submit data to be processed to a specified resource.
    /// It is typically used when creating new resources on the server or sending data
    /// that will cause a change in the server's state.
    POST,
}
