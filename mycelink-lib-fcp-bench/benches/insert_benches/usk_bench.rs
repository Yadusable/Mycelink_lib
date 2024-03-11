#![cfg(test)]

use criterion::Criterion;
use mycelink_lib_fcp::messages::client_put::ClientPutMessage;
use mycelink_lib_fcp::model::persistence::Persistence;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::model::upload_type::UploadType::Direct;
use mycelink_lib_fcp_bench::fcp_helper::{
    generate_ssk, measure_put_time, measure_usk_update_time_10, prepare_connection, FCP_PORT_1,
};
use rand::RngCore;

pub fn usk_bench(c: &mut Criterion) {
    println!("Prepare USK Update test");
    let (update_insert_uri, update_request_uri) = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(prepare_for_update_test());
    println!("Finished Preparations");

    c.bench_function("usk_initial_bench", |b| {
        b.to_async(
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(usk_bench_initial_fn);
    })
    .bench_function("usk_update_bench", |b| {
        b.to_async(
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| async {
            usk_bench_update_10(update_insert_uri.clone(), update_request_uri.clone()).await
        })
    });
}

async fn usk_bench_initial_fn() {
    let mut data: [u8; 1024] = [0; 1024];
    rand::thread_rng().fill_bytes(&mut data);
    let (mut tx, mut rx) = prepare_connection(FCP_PORT_1).await;

    let keypair = generate_ssk(&mut tx, &mut rx).await;

    let mut insert_uri = keypair.insert_uri.replace("SSK@", "USK@");
    insert_uri.push_str("test/0");

    let mut request_uri = keypair.request_uri.replace("SSK@", "USK@");
    request_uri.push_str("test/0");

    let put_message = ClientPutMessage {
        early_encode: true,
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

    measure_put_time(put_message, request_uri.as_str().try_into().unwrap()).await;
}

async fn usk_bench_update_10(insert_uri: String, request_uri: String) {
    let mut data: [u8; 1024] = [0; 1024];
    rand::thread_rng().fill_bytes(&mut data);

    let put_message = ClientPutMessage {
        early_encode: true,
        uri: insert_uri.as_str().try_into().unwrap(),
        content_type: None,
        identifier: UniqueIdentifier::new("Bench update USK"),
        verbosity: Default::default(),
        max_retries: 3,
        priority: PriorityClass::High,
        get_only_chk: false,
        dont_compress: true,
        persistence: Persistence::Connection,
        target_filename: None,
        upload_from: Direct { data: data.into() },
        is_binary_blob: false,
        real_time: true,
    };

    measure_usk_update_time_10(put_message, request_uri.as_str().try_into().unwrap()).await;
}

async fn prepare_for_update_test() -> (String, String) {
    let (mut tx, mut rx) = prepare_connection(FCP_PORT_1).await;
    let keypair = generate_ssk(&mut tx, &mut rx).await;

    let mut insert_uri = keypair.insert_uri.replace("SSK@", "USK@");
    insert_uri.push_str("test/0");

    let mut request_uri = keypair.request_uri.replace("SSK@", "USK@");
    request_uri.push_str("test/0");

    let put_message = ClientPutMessage {
        early_encode: true,
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
        upload_from: Direct {
            data: [0; 1024].into(),
        },
        is_binary_blob: false,
        real_time: true,
    };

    measure_put_time(put_message, request_uri.as_str().try_into().unwrap()).await;

    (insert_uri, request_uri)
}
