#[macro_use]
extern crate criterion;
extern crate nes;

use criterion::Criterion;
// use criterion::black_box;
use nes::mem::*;

fn ram_store_benchmark(c: &mut Criterion) {
  c.bench_function("ram-storeb-static-address", |b| b.iter(|| {
    let mut ram = Ram { val: [0; 0x800] };
    let address: u16 = 0x2000;

    ram.storeb(address, 200)
  }));

  c.bench_function("ram-storeb-random-address", |b| b.iter(|| {
    let mut ram = Ram { val: [0; 0x800] };

  }));
}

criterion_group!(benches, ram_store_benchmark);
criterion_main!(benches);