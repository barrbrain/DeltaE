extern crate criterion;
extern crate delta_e;
extern crate lab;

use criterion::*;
use delta_e::DE2000;
use lab::Lab;

fn de2000(c: &mut Criterion) {
    c.bench_function(&"DE2000::new", move |b| de2000_new(b));
    c.bench_function(&"DE2000::from_rgb", move |b| de2000_from_rgb(b));
}

fn de2000_from_rgb(b: &mut Bencher) {
    let color_1 = [234, 76, 76];
    let color_2 = [76, 187, 234];

    b.iter(|| DE2000::from_rgb(&color_1, &color_2));
}

fn de2000_new(b: &mut Bencher) {
    let color_1 = Lab {
        l: 38.972,
        a: 58.991,
        b: 37.138,
    };
    let color_2 = Lab {
        l: 54.528,
        a: 42.416,
        b: 54.497,
    };

    b.iter(|| DE2000::new(color_1, color_2));
}

criterion_group!(benches, de2000);
criterion_main!(benches);
