// Necessary for normally using RSESS
use rsess::{ESS, OESS, DistributionMatcher, ASK};
use rug::Integer;
// Additional for this example
use rug::rand::RandState;
use std::panic::catch_unwind;

fn main() {
    // Create an ESS instance with e_max == 100, n_max == 10 and 8-ASK
    let ess = ESS::new(100, 10, ASK::new(8));

    let data = Integer::from(1000);
    println!("Index: {}", data);

    // Convert the index to an amplitude sequence
    let amplitude_sequence = ess.encode(&data).unwrap();
    println!("Amplitude sequence: {:?}", amplitude_sequence);

    // Convert the amplitude sequence back to an index
    let recovered_data = ess.decode(&amplitude_sequence).unwrap();
    println!("Recovered index: {}", recovered_data);
    println!();



    // OESS constructor panics if a non optimum e_max is supplied
    let result = catch_unwind(|| {
        OESS::new(100, 10, ASK::new(8));
    });
    match result {
        Ok(_) => println!("All ok!"),
        Err(_) => println!("\nOESS constructor panicked because OESS is not defined for the provided `e_max` value!"),
    };

    // Use [OESS::optimal_e_max] to find the next lower valid e_max value
    let valid_e_max = OESS::optimal_e_max(100, 10, &ASK::new(8));
    OESS::new(valid_e_max, 10, ASK::new(8));
    println!("Using OESS::optimal_e_max, OESS constructor does not panic.");
    println!();



    // Encode / decode multiple random data values
    let num_transmissions = 100;
    let mut rand = RandState::new();
    let num_bits = ess.num_data_bits();

    let data: Vec<Integer> = (0..num_transmissions).map(|_| {
        Integer::from(Integer::random_bits(num_bits, &mut rand))
    }).collect();

    let mut sequences = Vec::with_capacity(data.len());
    for idx in data.iter() {
        sequences.push(ess.encode(&idx).unwrap());
    }

    let mut decoded_data = Vec::with_capacity(data.len());
    for amp_seq in sequences {
        decoded_data.push(ess.decode(&amp_seq).unwrap());
    }

    for (idx, decoded_idx) in data.iter().zip(decoded_data) {
        // panic if decoded data is unequal to sent data
        assert_eq!(*idx, decoded_idx);
    }
    println!("Encoding / decoding successful for all {} transmissions.", num_transmissions);
    println!();


    // Calculate the amplitude distribution and average energy
    println!("Amplitude distribution:");
    let amplitude_distribution =  ess.amplitude_distribution();
    let amplitudes = vec![1, 3, 5, 7];
    for (amplitude, probability) in amplitudes.iter().zip(amplitude_distribution) {
        println!("  P({}) = {}", amplitude, probability);
    }
    println!("Average energy: {:?}", ess.average_energy());
}
