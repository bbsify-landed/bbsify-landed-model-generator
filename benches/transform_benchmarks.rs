use criterion::{black_box, criterion_group, criterion_main, Criterion};
use model_generator::{Model, Transform};
use model_generator::primitives::{Cube, Sphere};
use model_generator::transforms::{Scale, Rotate, Translate, Mirror};

fn create_test_cube() -> Model {
    Cube::new()
        .size(1.0)
        .center(0.0, 0.0, 0.0)
        .build()
}

fn create_test_sphere() -> Model {
    Sphere::new()
        .radius(1.0)
        .segments(32)
        .rings(16)
        .build()
}

fn bench_scale_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("Scale Transform");
    
    // Benchmark uniform scaling
    group.bench_function("uniform_scale_cube", |b| {
        b.iter(|| {
            let mut model = create_test_cube();
            black_box(Scale::uniform(2.0).apply(&mut model))
        })
    });
    
    // Benchmark non-uniform scaling
    group.bench_function("non_uniform_scale_cube", |b| {
        b.iter(|| {
            let mut model = create_test_cube();
            black_box(Scale::new(1.0, 2.0, 3.0).apply(&mut model))
        })
    });
    
    // Benchmark scaling a complex model (sphere)
    group.bench_function("scale_sphere", |b| {
        b.iter(|| {
            let mut model = create_test_sphere();
            black_box(Scale::uniform(2.0).apply(&mut model))
        })
    });
    
    group.finish();
}

fn bench_rotate_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rotate Transform");
    
    // Benchmark rotation around Y axis
    group.bench_function("rotate_y_cube", |b| {
        b.iter(|| {
            let mut model = create_test_cube();
            black_box(Rotate::around_y(45.0).apply(&mut model))
        })
    });
    
    // Benchmark rotation around custom axis
    group.bench_function("rotate_custom_axis_cube", |b| {
        b.iter(|| {
            let mut model = create_test_cube();
            let axis = nalgebra::Vector3::new(1.0, 1.0, 1.0).normalize();
            black_box(Rotate::new(axis, 45.0).apply(&mut model))
        })
    });
    
    // Benchmark rotating a complex model (sphere)
    group.bench_function("rotate_sphere", |b| {
        b.iter(|| {
            let mut model = create_test_sphere();
            black_box(Rotate::around_y(45.0).apply(&mut model))
        })
    });
    
    group.finish();
}

fn bench_translate_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("Translate Transform");
    
    // Benchmark translation
    group.bench_function("translate_cube", |b| {
        b.iter(|| {
            let mut model = create_test_cube();
            black_box(Translate::new(1.0, 2.0, 3.0).apply(&mut model))
        })
    });
    
    // Benchmark translating a complex model (sphere)
    group.bench_function("translate_sphere", |b| {
        b.iter(|| {
            let mut model = create_test_sphere();
            black_box(Translate::new(1.0, 2.0, 3.0).apply(&mut model))
        })
    });
    
    group.finish();
}

fn bench_mirror_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mirror Transform");
    
    // Benchmark mirroring
    group.bench_function("mirror_cube", |b| {
        b.iter(|| {
            let mut model = create_test_cube();
            black_box(Mirror::new(true, true, true).apply(&mut model))
        })
    });
    
    // Benchmark mirroring a complex model (sphere)
    group.bench_function("mirror_sphere", |b| {
        b.iter(|| {
            let mut model = create_test_sphere();
            black_box(Mirror::new(true, true, true).apply(&mut model))
        })
    });
    
    group.finish();
}

fn bench_transform_chain(c: &mut Criterion) {
    let mut group = c.benchmark_group("Transform Chain");
    
    // Benchmark chaining multiple transforms
    group.bench_function("chain_transforms_cube", |b| {
        b.iter(|| {
            let mut model = create_test_cube();
            model.apply(Scale::uniform(2.0));
            model.apply(Rotate::around_y(45.0));
            model.apply(Translate::new(1.0, 2.0, 3.0));
            black_box(model.mesh.vertices.len())
        })
    });
    
    // Benchmark chaining transforms on a complex model (sphere)
    group.bench_function("chain_transforms_sphere", |b| {
        b.iter(|| {
            let mut model = create_test_sphere();
            model.apply(Scale::uniform(2.0));
            model.apply(Rotate::around_y(45.0));
            model.apply(Translate::new(1.0, 2.0, 3.0));
            black_box(model.mesh.vertices.len())
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_scale_transform,
    bench_rotate_transform,
    bench_translate_transform,
    bench_mirror_transform,
    bench_transform_chain
);
criterion_main!(benches); 