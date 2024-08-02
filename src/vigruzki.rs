#![allow(dead_code)]
#![allow(unused_imports)]
use log::{debug, warn};
use std::io::{Read, Write};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "SOAP-ENV"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getLastDumpDate")]
    pub struct GetLastDumpDate {
        #[yaserde(flatten, default)]
        pub parameters: types::GetLastDumpDate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getLastDumpDateResponse")]
    pub struct GetLastDumpDateResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::GetLastDumpDateResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getLastDumpDateEx")]
    pub struct GetLastDumpDateEx {
        #[yaserde(flatten, default)]
        pub parameters: types::GetLastDumpDateEx,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getLastDumpDateExResponse")]
    pub struct GetLastDumpDateExResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::GetLastDumpDateExResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "sendRequest")]
    pub struct SendRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::SendRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "sendRequestResponse")]
    pub struct SendRequestResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::SendRequestResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getResult")]
    pub struct GetResult {
        #[yaserde(flatten, default)]
        pub parameters: types::GetResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getResultSocResources")]
    pub struct GetResultSocResources {
        #[yaserde(flatten, default)]
        pub parameters: types::GetResultSocResources,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getResultResponse")]
    pub struct GetResultResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::GetResultResponse,
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getLastDumpDate",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct GetLastDumpDate {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getLastDumpDateResponse",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct GetLastDumpDateResponse {
        #[yaserde(rename = "lastDumpDate", prefix = "ns1", default)]
        pub last_dump_date: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getLastDumpDateEx",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct GetLastDumpDateEx {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getLastDumpDateExResponse",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct GetLastDumpDateExResponse {
        #[yaserde(rename = "lastDumpDate", prefix = "ns1", default)]
        pub last_dump_date: i64,
        #[yaserde(rename = "lastDumpDateUrgently", prefix = "ns1", default)]
        pub last_dump_date_urgently: i64,
        #[yaserde(rename = "lastDumpDateSocResources", prefix = "ns1", default)]
        pub last_dump_date_soc_resources: i64,
        #[yaserde(rename = "webServiceVersion", prefix = "ns1", default)]
        pub web_service_version: String,
        #[yaserde(rename = "dumpFormatVersion", prefix = "ns1", default)]
        pub dump_format_version: String,
        #[yaserde(rename = "dumpFormatVersionSocResources", prefix = "ns1", default)]
        pub dump_format_version_soc_resources: String,
        #[yaserde(rename = "docVersion", prefix = "ns1", default)]
        pub doc_version: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "sendRequest",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct SendRequest {
        #[yaserde(rename = "requestFile", prefix = "ns1", default)]
        pub request_file: String,
        #[yaserde(rename = "signatureFile", prefix = "ns1", default)]
        pub signature_file: String,
        #[yaserde(rename = "dumpFormatVersion", prefix = "ns1", default)]
        pub dump_format_version: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "sendRequestResponse",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct SendRequestResponse {
        #[yaserde(rename = "result", prefix = "ns1", default)]
        pub result: bool,
        #[yaserde(rename = "resultComment", prefix = "ns1", default)]
        pub result_comment: Option<String>,
        #[yaserde(rename = "code", prefix = "ns1", default)]
        pub code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getResult",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct GetResult {
        #[yaserde(rename = "code", prefix = "ns1", default)]
        pub code: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getResultSocResources",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct GetResultSocResources {
        #[yaserde(rename = "code", prefix = "ns1", default)]
        pub code: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getResultResponse",
        namespace = "ns1: http://vigruzki.rkn.gov.ru/OperatorRequest/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "ns1"
    )]
    pub struct GetResultResponse {
        #[yaserde(rename = "result", prefix = "ns1", default)]
        pub result: bool,
        #[yaserde(rename = "resultComment", prefix = "ns1", default)]
        pub result_comment: Option<String>,
        #[yaserde(rename = "registerZipArchive", prefix = "ns1", default)]
        pub register_zip_archive: Option<String>,
        #[yaserde(rename = "resultCode", prefix = "ns1", default)]
        pub result_code: i32,
        #[yaserde(rename = "dumpFormatVersion", prefix = "ns1", default)]
        pub dump_format_version: Option<String>,
        #[yaserde(rename = "operatorName", prefix = "ns1", default)]
        pub operator_name: Option<String>,
        #[yaserde(rename = "inn", prefix = "ns1", default)]
        pub inn: Option<String>,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type GetLastDumpDate = messages::GetLastDumpDate;

    pub type GetLastDumpDateResponse = messages::GetLastDumpDateResponse;

    pub type GetLastDumpDateEx = messages::GetLastDumpDateEx;

    pub type GetLastDumpDateExResponse = messages::GetLastDumpDateExResponse;

    pub type SendRequest = messages::SendRequest;

    pub type SendRequestResponse = messages::SendRequestResponse;

    pub type GetResult = messages::GetResult;

    pub type GetResultResponse = messages::GetResultResponse;

    pub type GetResultSocResources = messages::GetResultSocResources;

    #[async_trait]
    pub trait OperatorRequestPortType {
        async fn get_last_dump_date(
            &self,
            get_last_dump_date: GetLastDumpDate,
        ) -> Result<GetLastDumpDateResponse, Option<SoapFault>>;
        async fn get_last_dump_date_ex(
            &self,
            get_last_dump_date_ex: GetLastDumpDateEx,
        ) -> Result<GetLastDumpDateExResponse, Option<SoapFault>>;
        async fn send_request(
            &self,
            send_request: SendRequest,
        ) -> Result<SendRequestResponse, Option<SoapFault>>;
        async fn get_result(
            &self,
            get_result: GetResult,
        ) -> Result<GetResultResponse, Option<SoapFault>>;
        async fn get_result_soc_resources(
            &self,
            get_result_soc_resources: GetResultSocResources,
        ) -> Result<GetResultResponse, Option<SoapFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl OperatorRequestPortBinding {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetLastDumpDate {
        #[yaserde(rename = "getLastDumpDate", prefix = "ns1", default)]
        pub body: ports::GetLastDumpDate,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct GetLastDumpDateSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapGetLastDumpDate,
    }

    impl GetLastDumpDateSoapEnvelope {
        pub fn new(body: SoapGetLastDumpDate) -> Self {
            GetLastDumpDateSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetLastDumpDateResponse {
        #[yaserde(rename = "getLastDumpDateResponse", prefix = "ns1", default)]
        pub body: ports::GetLastDumpDateResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct GetLastDumpDateResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapGetLastDumpDateResponse,
    }

    impl GetLastDumpDateResponseSoapEnvelope {
        pub fn new(body: SoapGetLastDumpDateResponse) -> Self {
            GetLastDumpDateResponseSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetLastDumpDateEx {
        #[yaserde(rename = "getLastDumpDateEx", prefix = "ns1", default)]
        pub body: ports::GetLastDumpDateEx,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct GetLastDumpDateExSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapGetLastDumpDateEx,
    }

    impl GetLastDumpDateExSoapEnvelope {
        pub fn new(body: SoapGetLastDumpDateEx) -> Self {
            GetLastDumpDateExSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetLastDumpDateExResponse {
        #[yaserde(rename = "getLastDumpDateExResponse", prefix = "ns1", default)]
        pub body: ports::GetLastDumpDateExResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct GetLastDumpDateExResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapGetLastDumpDateExResponse,
    }

    impl GetLastDumpDateExResponseSoapEnvelope {
        pub fn new(body: SoapGetLastDumpDateExResponse) -> Self {
            GetLastDumpDateExResponseSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSendRequest {
        #[yaserde(rename = "sendRequest", prefix = "ns1", default)]
        pub body: ports::SendRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct SendRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapSendRequest,
    }

    impl SendRequestSoapEnvelope {
        pub fn new(body: SoapSendRequest) -> Self {
            SendRequestSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSendRequestResponse {
        #[yaserde(rename = "sendRequestResponse", prefix = "ns1", default)]
        pub body: ports::SendRequestResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct SendRequestResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapSendRequestResponse,
    }

    impl SendRequestResponseSoapEnvelope {
        pub fn new(body: SoapSendRequestResponse) -> Self {
            SendRequestResponseSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetResult {
        #[yaserde(rename = "getResult", prefix = "ns1", default)]
        pub body: ports::GetResult,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct GetResultSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapGetResult,
    }

    impl GetResultSoapEnvelope {
        pub fn new(body: SoapGetResult) -> Self {
            GetResultSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetResultResponse {
        #[yaserde(rename = "getResultResponse", default)]
        pub body: ports::GetResultResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct GetResultResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapGetResultResponse,
    }

    impl GetResultResponseSoapEnvelope {
        pub fn new(body: SoapGetResultResponse) -> Self {
            GetResultResponseSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetResultSocResources {
        #[yaserde(rename = "getResultSocResources", prefix = "ns1", default)]
        pub body: ports::GetResultSocResources,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "SOAP-ENV"
    )]
    pub struct GetResultSocResourcesSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "ns1", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
        pub body: SoapGetResultSocResources,
    }

    impl GetResultSocResourcesSoapEnvelope {
        pub fn new(body: SoapGetResultSocResources) -> Self {
            GetResultSocResourcesSoapEnvelope {
                encoding_style: Option::Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for OperatorRequestPortBinding {
        fn default() -> Self {
            OperatorRequestPortBinding {
                client: reqwest::Client::new(),
                url: "http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl OperatorRequestPortBinding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            OperatorRequestPortBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct OperatorRequestPortBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::OperatorRequestPortType for OperatorRequestPortBinding {
        async fn get_last_dump_date(
            &self,
            get_last_dump_date: ports::GetLastDumpDate,
        ) -> Result<ports::GetLastDumpDateResponse, Option<SoapFault>> {
            let __request = GetLastDumpDateSoapEnvelope::new(SoapGetLastDumpDate {
                body: get_last_dump_date,
                xmlns: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://vigruzki.rkn.gov.ru/services/OperatorRequest/getLastDumpDate",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetLastDumpDateResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_last_dump_date_ex(
            &self,
            get_last_dump_date_ex: ports::GetLastDumpDateEx,
        ) -> Result<ports::GetLastDumpDateExResponse, Option<SoapFault>> {
            let __request = GetLastDumpDateExSoapEnvelope::new(SoapGetLastDumpDateEx {
                body: get_last_dump_date_ex,
                xmlns: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://vigruzki.rkn.gov.ru/services/OperatorRequest/getLastDumpDateEx",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetLastDumpDateExResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn send_request(
            &self,
            send_request: ports::SendRequest,
        ) -> Result<ports::SendRequestResponse, Option<SoapFault>> {
            let __request = SendRequestSoapEnvelope::new(SoapSendRequest {
                body: send_request,
                xmlns: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://vigruzki.rkn.gov.ru/services/OperatorRequest/sendRequest",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: SendRequestResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_result(
            &self,
            get_result: ports::GetResult,
        ) -> Result<ports::GetResultResponse, Option<SoapFault>> {
            let __request = GetResultSoapEnvelope::new(SoapGetResult {
                body: get_result,
                xmlns: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://vigruzki.rkn.gov.ru/services/OperatorRequest/getResult",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetResultResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_result_soc_resources(
            &self,
            get_result_soc_resources: ports::GetResultSocResources,
        ) -> Result<ports::GetResultResponse, Option<SoapFault>> {
            let __request = GetResultSocResourcesSoapEnvelope::new(SoapGetResultSocResources {
                body: get_result_soc_resources,
                xmlns: Option::Some("http://vigruzki.rkn.gov.ru/OperatorRequest/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://vigruzki.rkn.gov.ru/services/OperatorRequest/getResultSocResources",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetResultResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub struct OperatorRequestService {}
    impl OperatorRequestService {
        pub fn new_client(
            credentials: Option<(String, String)>,
        ) -> bindings::OperatorRequestPortBinding {
            bindings::OperatorRequestPortBinding::new(
                "https://vigruzki.rkn.gov.ru/services/OperatorRequest/",
                credentials,
            )
        }
    }
}
