/*
 * @Author: Jerry.Yang
 * @Date: 2024-11-13 10:46:06
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:38:17
 * @Description: operation name clb
 */

/// Enum representing the operation names related to the CLB (Cloud Load Balancer) service.
/// This enum provides a type - safe way to define and refer to different operations that can be
/// performed on the CLB service. The `Debug` derive allows for easy debugging by providing
/// a default implementation of the `fmt::Debug` trait, enabling the enum variants to be printed
/// in a human - readable format. The `Clone` derive allows creating copies of the enum values.
#[derive(Debug, Clone)]
pub enum OperationNameClb {
    /// Represents the operation of retrieving information about load balancers in the CLB service.
    /// This operation is typically used to get details such as the configuration, status, and associated
    /// resources of the load balancers.
    DescribeLoadBalancers,
}

/// Implementation of the `ToString` trait for the `OperationNameClb` enum.
/// This implementation allows converting an instance of `OperationNameClb` into a string representation.
/// This is useful when the operation name needs to be passed as a parameter in an API call or logged.
impl ToString for OperationNameClb {
    /// Converts an `OperationNameClb` instance into a string.
    ///
    /// # Returns
    /// - A `String` representing the operation name. For the `DescribeLoadBalancers` variant,
    ///   it returns the string "DescribeLoadBalancers".
    fn to_string(&self) -> String {
        match self {
            // Map the `DescribeLoadBalancers` variant to the corresponding string
            OperationNameClb::DescribeLoadBalancers => "DescribeLoadBalancers",
        }
        // Convert the string literal to a `String` type
        .to_string()
    }
}
