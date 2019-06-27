#[macro_use]
extern crate criterion;
extern crate nes;

use criterion::Criterion;
use nes::mem::*;

fn ram_store_benchmark(c: &mut Criterion) {
    let mut ram = Ram { val: [0; 0x800] };
    let address: u16 = 0x2000;

    c.bench_function("ram-storeb-static-address", move |b| b.iter(|| {
        ram.storeb(address, 200);
    }));
}

fn ram_load_benchmark(c: &mut Criterion) {
    let mut ram = Ram { val: [0; 0x800] };
    let address: u16 = 0x2000;

    ram.storeb(address, 200);

    c.bench_function("ram-storeb-static-address", move |b| b.iter(|| {
        ram.loadb(address);
    }));
}

criterion_group!(benches, ram_store_benchmark, ram_load_benchmark);
criterion_main!(benches);