use cornucopi::{conn::cornucopi_conn, CodegenSettings};
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    cornucopi::container::cleanup(false).ok();
    cornucopi::container::setup(false).unwrap();
    let client = &mut cornucopi_conn().unwrap();

    cornucopi::load_schema(client, &["../codegen_test/schema.sql"]).unwrap();
    c.bench_function("codegen_sync", |b| {
        b.iter(|| {
            cornucopi::generate_live(
                client,
                "../test_codegen/queries",
                None,
                CodegenSettings {
                    gen_sync: true,
                    gen_async: false,
                    derive_ser: true,
                },
            )
            .unwrap()
        })
    });
    c.bench_function("codegen_async", |b| {
        b.iter(|| {
            cornucopi::generate_live(
                client,
                "../test_codegen/queries",
                None,
                CodegenSettings {
                    gen_sync: true,
                    gen_async: false,
                    derive_ser: true,
                },
            )
            .unwrap()
        })
    });
    cornucopi::container::cleanup(false).unwrap();
}
criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
