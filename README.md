<!--
 * @Author: Jerry.Yang
 * @Date: 2025-02-20 17:45:24
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2025-02-20 18:53:22
 * @Description: readme
-->

---

## 依赖

- **tokio**: 用于异步运行时。
- **volcengine_sdk_protobuf**: Volcengine SDK 的 Protobuf 定义。

---

## 调用示例说明

### 示例函数 `ecs_describe_instances`

该测试函数用于测试通过 Volcengine SDK 查询 Redis 实例的功能。以下是测试的主要步骤：

1. **配置认证信息**：使用 `access_key_id` 和 `secret_access_key` 创建 `Credentials` 对象。
2. **创建配置**：使用 `Config::builder()` 创建配置对象，并设置 `region_id` 和 `credentials`。
3. **创建会话**：使用 `Session::builder()` 创建会话对象，并传入配置。
4. **创建 Ecs 服务**：使用 `Ecs::new_ecs()` 创建 Ecs 服务对象。
5. **构建请求**：创建一个 `DescribeInstancesReq` 请求对象。
6. **执行查询**：调用 `ecs.new_describe_instances(request).await` 异步方法查询 Redis 实例。

---

### 运行测试

要运行测试，请确保已正确配置 `access_key_id` 和 `secret_access_key`，然后在项目根目录下运行以下命令：

```bash
use volcengine_rust_sdk::service::ecs;
use volcengine_rust_sdk::service::ecs::EcsService;
use volcengine_rust_sdk::volcengine::config;
use volcengine_rust_sdk::volcengine::credentials::credentials;
use volcengine_rust_sdk::volcengine::session::session;
use volcengine_rust_sdk::volcengine::error::error;

async fn ecs_describe_instances() -> std::result::Result<volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesResp, error::Error> {
    // 1. 配置认证信息
    let access_key_id = ""; // 这里填入实际的 Access Key ID
    let secret_access_key = ""; // 这里填入实际的 Secret Access Key
    let region_id = ""; // 这里填入实际的 Region ID
    let credentials = credentials::Credentials::new(access_key_id, secret_access_key);

    // 2. 创建配置
    let config = config::Config::builder()
        .with_region(&region_id)
        .with_credentials(credentials)
        .build()?;

    // 3. 创建会话
    let session = session::Session::builder().with_config(config).build()?;

    // 4. 创建 ecs 服务
    let ecs = ecs::Ecs::new_ecs(session)?;

    // 5. 构建请求
    let mut request = volcengine_sdk_protobuf::protobuf::ecs_instance::DescribeInstancesReq();

    // 6. 执行查询
    let result = ecs.new_describe_instances(request).await?;
    
    // 7.返回
    return Ok(result)
}

```


# volcengine-rust-sdk
