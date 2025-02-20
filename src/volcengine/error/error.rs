use reqwest::header;
use thiserror::Error;
use url::ParseError;

/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-21 14:20:07
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-11-18 16:50:08
 * @Description: error
 */
#[derive(Error, Debug)]
pub enum Error {
    // util config
    #[error("util config Err : build Config no credentials")]
    ErrUtilConfigBuildConfigNoCredentials,

    // util session
    #[error("util session Err : build session no config")]
    ErrUtilSessionBuildSessionNoConfig,

    // util request
    #[error("util request Err : build request no {0}")]
    ErrUtilRequestBuildRequestNo(String),
    #[error("util request Err : build operation no {0}")]
    ErrUtilRequestBuildOperationNo(String),

    // util client
    #[error("util client Err : build clientInfo no {0}")]
    ErrUtilClientBuildClientInfoNo(String),
    #[error("util client Err : build client no {0}")]
    ErrUtilClientBuildClientNo(String),

    // request
    #[error("request Err : {0}")]
    ErrRequest(#[from] reqwest::Error),
    #[error("parse Err : {0}")]
    ErrParse(#[from] ParseError),
    #[error("request Err : requestBuilder Is None")]
    ErrRequestBuilderIsNone,
    #[error("request Err : Invalid method")]
    ErrRequestInvalidMethod,
    #[error("request Err : request build failed")]
    ErrRequestBuildFailed,
    #[error("request Err : header is Err : {0}")]
    ErrRequestHeaderIsErr(#[from] header::ToStrError),
    #[error("request Err : status is {0}")]
    ErrResponseStatus(reqwest::StatusCode),
    #[error("Failed to parse response: {0}")]
    ErrParseResponse(reqwest::Error),
    #[error("Fail to parse json : {0}")]
    ErrParseJson(serde_json::Error),
    // request sign
    #[error("request sign Err : {0}")]
    ErrRequestSignGetHost(url::ParseError),
    #[error("request sign Err : get host not found")]
    ErrRequestSignGetHostNone,
    #[error("request sign Err : get header-{0} not found")]
    ErrRequestSignGetHeaderNone(String),
}
