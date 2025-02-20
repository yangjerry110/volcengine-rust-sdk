/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-22 13:52:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-20 16:19:10
 * @Description: create_user test
 */
#[cfg(test)]
mod tests {
    use volcengine_sdk_protobuf::protobuf::vpc_subnet;

    use crate::{
        service::vpc::{self, VpcService},
        volcengine::{config, credentials::credentials, session::session},
    };

    #[tokio::test]
    async fn test_get_user() {
        let access_key_id = ""; // 这里填入实际的 Access Key ID
        let secret_access_key = ""; // 这里填入实际的 Secret Access Key
        let region_id = "cn-beijing"; // 这里填入实际的 Region ID

        let credentials = credentials::Credentials::new(access_key_id, secret_access_key);

        // new config
        let config = config::Config::builder()
            .with_region(&region_id)
            .with_credentials(credentials)
            .build();

        // print config
        println!("config : {:?}", config);

        // reset config
        let config = config.unwrap();

        // new session
        let session = session::Session::builder().with_config(config).build();

        // print session
        println!("session : {:?}", session);

        // reset session
        let session = session.unwrap();

        // new vpc
        let vpc = vpc::Vpc::new_vpc(session);

        // print vpc
        println!("vpc : {:?}", vpc);

        // reset vpc
        let vpc = vpc.unwrap();

        // create_user
        let mut request = vpc_subnet::DescribeSubnetsReq::default();
        request.vpc_id = Some("".to_string());
        request.zone_id = Some("".to_string());

        // 运行异步代码的 helper 函数
        let result = vpc.new_describe_subnets(request).await;

        println!("result : {:?}", result);

        // 这里可以添加断言来检查结果
        assert!(result.is_ok());
    }
}
