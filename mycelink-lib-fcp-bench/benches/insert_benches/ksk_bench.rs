#![cfg(test)]

use criterion::Criterion;
use mycelink_lib_fcp::messages::client_put::ClientPutMessage;
use std::time::UNIX_EPOCH;

use mycelink_lib_fcp::model::persistence::Persistence;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::model::upload_type::UploadType::Direct;
use mycelink_lib_fcp::model::uri::URI;
use mycelink_lib_fcp_bench::fcp_helper::measure_put_time;
use rand::RngCore;

pub fn ksk_bench(c: &mut Criterion) {
    c.bench_function("usk_initial_bench", |b| {
        b.to_async(
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(ksk_bench_initial_fn);
    });
}

async fn ksk_bench_initial_fn() {
    let mut data: [u8; 1024] = [0; 1024];
    rand::thread_rng().fill_bytes(&mut data);

    let test_tag = UNIX_EPOCH.elapsed().unwrap().as_millis();
    let uri: URI = format!("KSK@my_little_benchmark_age-{test_tag}")
        .as_str()
        .try_into()
        .unwrap();

    let put_message = ClientPutMessage {
        early_encode: false,
        uri: uri.clone(),
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

    measure_put_time(put_message, uri).await;
}
