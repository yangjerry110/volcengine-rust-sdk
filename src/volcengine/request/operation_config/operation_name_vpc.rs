/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 10:56:49
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:46:34
 * @Description: operation name vpc
 */

/// Enum representing operation names for the Virtual Private Cloud (VPC) service.
/// This enum provides a type - safe way to define and refer to different operations
/// that can be performed on VPC resources. The `Debug` derive allows for easy debugging
/// by providing a default implementation of the `fmt::Debug` trait, enabling enum variants
/// to be printed in a human - readable format. The `Clone` derive allows creating copies
/// of the enum values.
#[derive(Debug, Clone)]
pub enum OperationNameVpc {
    /// Represents the operation of retrieving information about Virtual Private Clouds (VPCs).
    /// This operation can be used to get details such as VPC names, CIDR blocks, associated
    /// route tables, and security groups. It helps in understanding the overall VPC
    /// configuration and status within the cloud environment.
    DescribeVpcs,
    /// Represents the operation of fetching information about subnets within a VPC.
    /// Subnets are subdivisions of a VPC's IP address range. This operation can return
    /// details like subnet CIDR blocks, availability zones, and associations with network
    /// access control lists. It is useful for managing and troubleshooting network
    /// segmentation within a VPC.
    DescribeSubnets,
}

/// Implementation of the `ToString` trait for the `OperationNameVpc` enum.
/// This implementation enables converting an `OperationNameVpc` instance into a string representation.
/// It is useful when passing the operation name as a parameter in API calls or for logging purposes.
impl ToString for OperationNameVpc {
    /// Converts an `OperationNameVpc` instance into a string.
    ///
    /// # Returns
    /// - A `String` corresponding to the operation name. Each enum variant maps to a specific string
    ///   that represents the actual operation name used in the VPC API.
    fn to_string(&self) -> String {
        match self {
            OperationNameVpc::DescribeSubnets => "DescribeSubnets",
            OperationNameVpc::DescribeVpcs => "DescribeVpcs",
        }
        // Convert the string literal to a `String` type
        .to_string()
    }
}
