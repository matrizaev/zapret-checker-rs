mod vigruzki;

use base64::prelude::*;
use clap::{arg, command, value_parser};
use core::panic;
use log::info;
use std::path::PathBuf;
use std::{env, fs};
use tokio;
use tokio::time::{sleep, Duration};
use vigruzki::{messages, ports::OperatorRequestPortType, services::OperatorRequestService, types};

const SLEEP_DURATION: Duration = Duration::from_secs(60);
const MAX_RETRIES: i32 = 5;
const DEFAULT_REQUEST_FILE_NAME: &str = "request.xml";
const DEFAULT_SIGNATURE_FILE_NAME: &str = "request.xml.sig";
const DEFAULT_DUMP_FILE_NAME: &str = "dump.xml.zip";
const DEFAULT_TIMESTAMP_FILE_NAME: &str = "timestamp";

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                -r --request <FILE> "Sets a custom request file. Default is request.xml"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -s --signature <FILE> "Sets a custom signature file. Default is request.xml.sig"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -t --timestamp <FILE> "Sets a custom timestamp file. Default is timestamp"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -d --dump <FILE> "Sets a custom dump file. Default is dump.xml.zip"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let default_request_file_name = PathBuf::from(DEFAULT_REQUEST_FILE_NAME);
    let request_file_name = matches
        .get_one::<PathBuf>("request")
        .unwrap_or(&default_request_file_name);

    let default_signature_file_name = PathBuf::from(DEFAULT_SIGNATURE_FILE_NAME);
    let signature_file_name = matches
        .get_one::<PathBuf>("signature")
        .unwrap_or(&default_signature_file_name);

    let default_dump_file_name = PathBuf::from(DEFAULT_DUMP_FILE_NAME);
    let dump_file_name = matches
        .get_one::<PathBuf>("dump")
        .unwrap_or(&default_dump_file_name);

    let default_timestamp_file_name = PathBuf::from(DEFAULT_TIMESTAMP_FILE_NAME);
    let timestamp_file_name = matches
        .get_one::<PathBuf>("timestamp")
        .unwrap_or(&default_timestamp_file_name);

    let previous_timestamp: i64 = fs::read_to_string(timestamp_file_name)
        .unwrap_or_default()
        .parse()
        .unwrap_or_default();

    let request_file = fs::read(request_file_name).expect("Unable to read the request file");
    let signature_file = fs::read(signature_file_name).expect("Unable to read the request file");

    let request_file = BASE64_STANDARD.encode(request_file);
    let signature_file = BASE64_STANDARD.encode(signature_file);

    let client = OperatorRequestService::new_client(None);

    let response = client
        .get_last_dump_date_ex(messages::GetLastDumpDateEx {
            parameters: types::GetLastDumpDateEx {},
        })
        .await
        .expect("can not get_last_dump_date_ex");

    if previous_timestamp == response.parameters.last_dump_date {
        info!("The last_dump_date hasn't changed. Exiting.");
        return;
    }

    fs::write(
        timestamp_file_name,
        response.parameters.last_dump_date.to_string(),
    )
    .expect("Unable to write to the timestamp file");

    info!(
        "The last_dump_date has changed to {}. Proceeding with the request.",
        response.parameters.last_dump_date
    );

    let response = client
        .send_request(messages::SendRequest {
            parameters: types::SendRequest {
                request_file: request_file.into(),
                signature_file: signature_file.into(),
                dump_format_version: Some(response.parameters.dump_format_version),
            },
        })
        .await
        .expect("can not send_request");

    let code;

    if response.parameters.result {
        match response.parameters.code {
            Some(c) => {
                info!("sendRequest was successful. Code: {}", c);
                code = c;
            }
            None => panic!("sendRequest was successful. But the code is missing."),
        }
    } else {
        panic!("send_request wasn't successful.");
    }

    for _ in 0..MAX_RETRIES {
        let response = client
            .get_result(messages::GetResult {
                parameters: types::GetResult { code: code.clone() },
            })
            .await
            .expect("can not get_result");

        if response.parameters.result {
            info!("getResult was successful. Saving the dump.xml");

            //Save the dump.xml to a file
            let dump_xml = response.parameters.register_zip_archive.unwrap();
            let dump_xml = BASE64_STANDARD.decode(dump_xml).unwrap();

            fs::write(dump_file_name, dump_xml).expect("Unable to write to the dump file");
            info!("The dump.xml has been saved successfully.");
            return;
        } else {
            info!(
                "getResult wasn't successful. Retrying in {} seconds...",
                SLEEP_DURATION.as_secs()
            );
            sleep(SLEEP_DURATION).await;
        }
    }

    panic!("get_result wasn't successful.");
}
