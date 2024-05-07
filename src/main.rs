use crate::vigruzki::ports::OperatorRequestPortType;
use tokio;
use vigruzki::{messages, services::OperatorRequestService, types};

mod vigruzki;

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = OperatorRequestService::new_client(None);

    let response = client
        .get_last_dump_date(messages::GetLastDumpDate {
            parameters: types::GetLastDumpDate {},
        })
        .await
        .expect("can not get_last_dump_date_ex");
    println!("{:?}", response);
}
