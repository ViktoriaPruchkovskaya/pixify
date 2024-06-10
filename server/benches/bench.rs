use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pixify::embroidery::canvas::{Canvas, CanvasConfig};

fn bench_canvas_matrix(c: &mut Criterion) {
    let pic = black_box(include_bytes!("../tests/pic.png").to_vec());
    let config = black_box(
        CanvasConfig::new(pic.clone(), Some(30), Some(20)).expect("Failed to create config"),
    );
    c.bench_function("canvas matrix generation", |b| {
        b.iter(|| {
            let _ = Canvas::new(config.clone()).expect("Failed to create canvas");
        })
    });
}

fn bench_bytes_canvas(c: &mut Criterion) {
    let pic = black_box(include_bytes!("../tests/pic.png").to_vec());
    let config = black_box(
        CanvasConfig::new(pic.clone(), Some(30), Some(20)).expect("Failed to create config"),
    );
    c.bench_function("canvas bytes generation", |b| {
        b.iter(|| {
            let _ = Canvas::new(config.clone())
                .expect("Failed to create canvas")
                .get_bytes();
        })
    });
}

criterion_group!(benches, bench_canvas_matrix, bench_bytes_canvas);
criterion_main!(benches);
