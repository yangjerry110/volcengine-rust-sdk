/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-22 13:52:22
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-19 16:02:11
 * @Description: create_user test
 */
#[cfg(test)]
mod tests {
    use crate::{
        service::redis,
        service::redis::RedisService,
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

        // new ecs
        let redis = redis::Redis::new_redis(session);

        // print ecs
        println!("redis : {:?}", redis);

        // reset ecs
        let redis = redis.unwrap();

        // create_user
        let mut request =
            volcengine_sdk_protobuf::protobuf::redis_instance::RedisDescribeDbInstancesReq::default(
            );
        request.page_number = Some(10);
        request.page_size = Some(100);
        // request.auto_renew = Some(true);
        // request.auto_renew_period = Some(1);
        // request.count = Some(1);
        // // request.dry_run = Some(true);
        // request.image_id = Some("image-yde0q2u36nb6uofymxm5".to_string());
        // request.instance_charge_type = Some("PrePaid".to_string());
        // request.instance_name = Some("volcengine-bja-techcenter-pi-oa-api-[8,1]".to_string());
        // request.instance_type_id = Some("ecs.t2-c2m1.large".to_string());
        // request.key_pair_name = Some("ops".to_string());
        // request.period = Some(1);
        // request.project_name = Some("techcenter-pi-oa-api".to_string());
        // request.suffix_index = Some(8);
        // request.unique_suffix = Some(true);
        // request.zone_id = Some("cn-beijing-a".to_string());

        // // create instance volumes
        // let mut request_volumes: Vec<
        //     volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesVolumesReq,
        // > = Vec::new();
        // let mut request_volume =
        //     volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesVolumesReq::default();
        // let mut request_volume1 =
        //     volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesVolumesReq::default();
        // request_volume.size = Some(100);
        // // request_volume.volume_type = Some("ESSD_PL0".to_string());
        // request_volume1.size = Some(100);
        // request_volume1.volume_type = Some("Ultra_Disk".to_string());
        // request_volumes.push(request_volume);
        // request_volumes.push(request_volume1);
        // request.volumes = request_volumes;

        // // create instance tag
        // let mut request_tags: Vec<
        //     volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesTagsReq,
        // > = Vec::new();
        // let mut request_tag =
        //     volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesTagsReq::default();
        // request_tag.key = Some("cmdb_project_name".to_string());
        // request_tag.value = Some("techcenter-pi-oa-api".to_string());
        // request_tags.push(request_tag);
        // request.tags = request_tags;

        // // create instance networkinterfaces
        // let mut request_networkinterfaces: Vec<
        //     volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesNetworkInterfacesReq,
        // > = Vec::new();
        // let mut request_networkitnerface = volcengine_sdk_protobuf::protobuf::ecs_instance::RunInstancesNetworkInterfacesReq::default();
        // request_networkitnerface.security_group_ids =
        //     vec!["sg-rs4s5xddbmyov0x57zbxmf9".to_string()];
        // request_networkitnerface.subnet_id = Some("subnet-rrag8md0u22ov0x5890j4su".to_string());
        // request_networkinterfaces.push(request_networkitnerface);
        // request.network_interfaces = request_networkinterfaces;

        // 运行异步代码的 helper 函数
        let result = redis.new_describe_db_instances(request).await;

        println!("result : {:?}", result);

        // 这里可以添加断言来检查结果
        assert!(result.is_ok());
    }
}
