/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-22 13:52:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 16:03:36
 * @Description: create_user test
 */
#[cfg(test)]
mod tests {

    use crate::{
        service::rds::{self, RdsService},
        volcengine::{config, credentials::credentials, session::session},
    };

    #[tokio::test]
    async fn test_create_mysql() {
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

        // new rds
        let rds = rds::Rds::new_rds(session);

        // print rds
        println!("rds : {:?}", rds);

        // reset rds
        let rds = rds.unwrap();

        // create mysql
        let mut request =
            volcengine_sdk_protobuf::protobuf::rds_instance::DescribeDbInstanceDetailReq::default();
        request.instance_id = Some("mysql-04529706be3d".to_string());

        // 运行异步代码的 helper 函数
        let result = rds.new_describe_db_instance_detail(request).await;

        println!("result : {:?}", result);

        // 这里可以添加断言来检查结果
        assert!(result.is_ok());
    }
}
