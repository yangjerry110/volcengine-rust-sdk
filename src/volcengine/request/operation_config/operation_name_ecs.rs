/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-30 11:05:43
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-06 16:39:46
 * @Description: operation name ecs
 */

/// Enum representing various operation names for the Elastic Compute Service (ECS).
/// This enum provides a type - safe way to define and reference different operations
/// that can be performed on ECS resources. The `Debug` derive allows for easy debugging
/// by providing a default implementation of the `fmt::Debug` trait, enabling enum variants
/// to be printed in a human - readable format. The `Clone` derive allows creating copies
/// of the enum values.
#[derive(Debug, Clone)]
pub enum OperationNameEcs {
    /// Represents the operation of launching new ECS instances.
    /// This operation is used to create and start one or more virtual machines in the ECS service.
    RunInstances,
    /// Represents the operation of retrieving information about available ECS images.
    /// ECS images are templates that contain a pre - configured operating system and software,
    /// and this operation helps in getting details about these images.
    DescribeImages,
    /// Represents the operation of fetching information about existing ECS instances.
    /// It can be used to get details such as instance status, configuration, and associated metadata.
    DescribeInstances,
    /// Represents the operation of obtaining information about available regions in the ECS service.
    /// Regions are geographical areas where the cloud provider has data centers.
    DescribeRegions,
    /// Represents the operation of getting information about available availability zones within a region.
    /// Availability zones are distinct locations within a region that are isolated from each other in terms of failures.
    DescribeZones,
    /// Represents the operation of modifying the specification (e.g., CPU, memory) of an existing ECS instance.
    /// This allows adjusting the resources allocated to an instance according to changing requirements.
    ModifyInstanceSpec,
    /// Represents the operation of stopping a single ECS instance.
    /// This halts the running state of the specified instance.
    StopInstance,
    /// Represents the operation of stopping multiple ECS instances at once.
    /// It can be used to efficiently manage the state of a group of instances.
    StopInstances,
}

/// Implementation of the `ToString` trait for the `OperationNameEcs` enum.
/// This implementation enables converting an `OperationNameEcs` instance into a string representation.
/// It is useful when passing the operation name as a parameter in API calls or for logging purposes.
impl ToString for OperationNameEcs {
    /// Converts an `OperationNameEcs` instance into a string.
    ///
    /// # Returns
    /// - A `String` corresponding to the operation name. Each enum variant maps to a specific string
    ///   that represents the actual operation name used in the ECS API.
    fn to_string(&self) -> String {
        match self {
            OperationNameEcs::RunInstances => "RunInstances",
            OperationNameEcs::DescribeImages => "DescribeImages",
            OperationNameEcs::DescribeInstances => "DescribeInstances",
            OperationNameEcs::DescribeRegions => "DescribeRegions",
            OperationNameEcs::DescribeZones => "DescribeZones",
            OperationNameEcs::ModifyInstanceSpec => "ModifyInstanceSpec",
            OperationNameEcs::StopInstance => "StopInstance",
            OperationNameEcs::StopInstances => "StopInstances",
        }
        // Convert the string literal to a `String` type
        .to_string()
    }
}
