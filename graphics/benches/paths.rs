use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use nightgraphics::geometry::*;
use std::iter::*;

fn path_with_commands(c: &mut Criterion) {
    let mut group = c.benchmark_group("path::with_commands");
    let cmds_len = 3;
    for c in (1..=5).map(|n| {
        vec![
            PathEl::MoveTo(Point { x: 2., y: 2. }),
            PathEl::LineTo(Point { x: 4., y: 4. }),
            PathEl::CurveTo(
                Point { x: 6., y: 5. },
                Point { x: 7., y: 11. },
                Point { x: 15., y: 15. },
            ),
        ]
        .into_iter()
        .cycle()
        .take(cmds_len * n)
        .collect::<Vec<PathEl>>()
    }) {
        group.throughput(Throughput::Elements(c.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(c.len()), &c, |b, c| {
            b.iter(|| black_box(Path::from_commands(c)))
        });
    }
}

criterion_group!(benches, path_with_commands);
criterion_main!(benches);
