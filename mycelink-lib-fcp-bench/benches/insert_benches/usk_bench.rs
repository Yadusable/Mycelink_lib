#![cfg(test)]

use criterion::Criterion;
use mycelink_lib_fcp::messages::client_put::ClientPutMessage;
use mycelink_lib_fcp::messages::put_successful::PutSuccessfulMessage;
use mycelink_lib_fcp::messages::uri_generated::UriGeneratedMessage;
use mycelink_lib_fcp::model::message::{FCPEncodable, Message};
use mycelink_lib_fcp::model::persistence::Persistence;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::model::upload_type::UploadType::Direct;
use mycelink_lib_fcp_bench::fcp_helper::{generate_ssk, prepare_connection};
use rand::RngCore;
use tokio::io::AsyncWriteExt;

pub fn usk_bench(c: &mut Criterion) {
    c.bench_function("usk_initial_bench", |b| {
        b.to_async(
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| usk_bench_initial_fn());
    });
}

async fn usk_bench_initial_fn() {
    let mut data: [u8; 1024] = [0; 1024];
    rand::thread_rng().fill_bytes(&mut data);
    let (mut tx, mut rx) = prepare_connection().await;

    let keypair = generate_ssk(&mut tx, &mut rx).await;

    let mut insert_uri = keypair.insert_uri.replace("SSK@", "USK@");
    insert_uri.push_str("test/0");

    let put_message = ClientPutMessage {
        uri: insert_uri.as_str().try_into().unwrap(),
        content_type: None,
        identifier: UniqueIdentifier::new("Bench insert USK"),
        verbosity: Default::default(),
        max_retries: 0,
        priority: PriorityClass::High,
        get_only_chk: false,
        dont_compress: true,
        persistence: Persistence::Connection,
        target_filename: None,
        upload_from: Direct { data: data.into() },
        is_binary_blob: false,
        real_time: true,
    };

    let encoded = (&put_message).to_message().encode();
    tx.write_all(encoded.as_slice()).await.unwrap();

    let _uri_updated: UriGeneratedMessage =
        Message::decode(&mut rx).await.unwrap().try_into().unwrap();

    let _put_successful: PutSuccessfulMessage =
        Message::decode(&mut rx).await.unwrap().try_into().unwrap();
}
