/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-22 13:52:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-07 11:13:08
 * @Description: create_user test
 */
#[cfg(test)]
mod tests {
    use crate::{
        service::iam::{self, IamService},
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

        // new iam
        let iam = iam::Iam::new_iam(session);

        // print iam
        println!("iam : {:?}", iam);

        // reset iam
        let iam = iam.unwrap();

        // create_user
        let mut request = volcengine_sdk_protobuf::protobuf::iam_user::GetUserReq::default();
        request.user_name = "yangjie04".to_string();

        // 运行异步代码的 helper 函数
        let result = iam.new_get_user(request).await;

        println!("result : {:?}", result);

        // 这里可以添加断言来检查结果
        assert!(result.is_ok());
    }
}