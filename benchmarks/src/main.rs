use jemalloc_ctl::{stats, epoch};
use std::env;
use std::time::Instant;
use rug::Integer;
use rug::rand::RandState;
use json::object;
use chrono;

use rsess::{DistributionMatcher, ESS, OESS, ASK};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;


fn main() {
    let num_transmissions = 10000;

    let args: Vec<String> = env::args().collect();
    let e_max = usize::from_str_radix(&args[2], 10).unwrap();
    let n_max = usize::from_str_radix(&args[3], 10).unwrap();

    // Start RAM measurement ###############################
    epoch::advance().unwrap();
    let allocated_1 = stats::allocated::read().unwrap() as f32 / 1024.0;
    let resident_1 = stats::resident::read().unwrap() as f32 / 1024.0;

    let now = Instant::now();
    let dm: Box<dyn DistributionMatcher> = match args[1].as_str() {
        "OESS" => Box::new(OESS::new(e_max, n_max, ASK::new(8))),
        "ESS" => Box::new(ESS::new(e_max, n_max, ASK::new(8))),
        &_ => panic!("Illegal first argument! Must be 'ESS' or 'OESS'."),
    };
    let trellis_time_ms = now.elapsed().as_micros() as f64 / 1000.0;

    epoch::advance().unwrap();
    let allocated_2 = stats::allocated::read().unwrap() as f32 / 1024.0;
    let resident_2 = stats::resident::read().unwrap() as f32 / 1024.0;
    // End RAM measurement ################################

    let mut rand = RandState::new();
    let num_bits = dm.num_data_bits();

    let data: Vec<Integer> = (0..num_transmissions).map(|_| {
        Integer::from(Integer::random_bits(num_bits, &mut rand))
    }).collect();

    let mut sequences = Vec::with_capacity(data.len());
    let now = Instant::now();
    for idx in data.iter() {
        sequences.push(dm.encode(&idx).unwrap());
    }
    let encoding_ms = now.elapsed().as_micros() as f64 / 1000.0;

    let mut decoded_data = Vec::with_capacity(data.len());
    let now = Instant::now();
    for amp_seq in sequences {
        decoded_data.push(dm.decode(&amp_seq).unwrap());
    }
    let decoding_ms = now.elapsed().as_micros() as f64 / 1000.0;

    for (idx, decoded_idx) in data.iter().zip(decoded_data) {
        assert_eq!(*idx, decoded_idx);
    }

    let output = object!{
        tool: "rsess-bench",
        n_max: dm.n_max(),
        e_max: dm.e_max(),
        r_sh: dm.num_data_bits() as f64 / dm.n_max() as f64,
        class: args[1].as_str(),
        allocated_kB: allocated_2,
        resident_MB: resident_2 / 1024.0,
        diff_allocated_kB: allocated_2 - allocated_1,
        diff_resident_kB: resident_2 - resident_1,
        trellis_build_time_ms: trellis_time_ms,
        encoding_time_ms: encoding_ms,
        decoding_time_ms: decoding_ms,
        time_stamp: format!("{}", chrono::offset::Local::now().format("%d.%m.%Y %H:%M:%S")),
    };
    println!("{:#}", output);
}
