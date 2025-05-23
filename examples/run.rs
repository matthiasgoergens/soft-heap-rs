// Run as
//  RUST_MIN_STACK=16777216 cargo run --release --example run

use std::cmp::max;

use rand::seq::SliceRandom;
use softheap::pairing::SoftHeap;
use seq_macro::seq; // Add import for seq_macro

pub fn one_batch() {
    // let n = 10_000_000;
    const EVERY: usize = 32;
    println!("EVERY: {EVERY} one_batch random");
    for e in 0..27 {
        let n = 1 << e;

        let mut pairing: SoftHeap<EVERY, _> = SoftHeap::default();
        let mut x = (0..n).collect::<Vec<_>>();

        x.shuffle(&mut rand::rng());
        for i in x {
            pairing = pairing.insert(i);
        }
        let mut all_corrupted = 0;
        let mut max_corrupted = 0;

        // let mut c = 0;
        while !pairing.is_empty() {
            // c += 1;
            let (new_pairing, _item, newly_corrupted) = pairing.delete_min();
            all_corrupted += newly_corrupted.len();
            pairing = new_pairing;
                max_corrupted = max(max_corrupted, pairing.count_corrupted());
            // if !newly_corrupted.is_empty() {
            //     print!(
            //         "{c}\tTotal corrupted: {all_corrupted}\tNewly corrupted: {}\t",
            //         newly_corrupted.len()
            //     );
            //     let corrupted_fraction = all_corrupted as f64 / n as f64;
            //     println!("Corrupted fraction: {:.2}%", corrupted_fraction * 100.0);
            // }
            // println!(
            //     "Total corrupted: {all_corrupted}\tUncorrupted: {}\tCorrupted: {}",
            //     pairing.count_uncorrupted(),
            //     pairing.count_corrupted()
            // );
        }
        let ever_corrupted_fraction = all_corrupted as f64 / n as f64;
        print!(
            "Corrupted fraction: {:.2}%\te: {e}\tn: {n:10}\t",
            ever_corrupted_fraction * 100.0
        );
        println!("Max corrupted fraction: {:6.2}%", max_corrupted as f64 / n as f64 * 100.0);
    }
    // println!(
    //     "Total corrupted: {all_corrupted}\tUncorrupted: {}\tCorrupted: {}",
    //     pairing.count_uncorrupted(),
    //     pairing.count_corrupted()
    // );
}

pub fn interleave() {
    // let n = 10_000_000;
    const EVERY: usize = 32;
    println!("EVERY: {EVERY}");
    for e in 0..30 {
        let n = 1 << e;

        let mut pairing: SoftHeap<EVERY, _> = SoftHeap::default();

        let mut all_corrupted = 0;
        let mut non_corrupted_pops = 0;
        let mut c = 0;
        for _i in 0..n {
            c += 1;
            pairing = pairing.insert(c);
            // if pairing.root.as_ref().map(|r| r.children.len()).unwrap_or(0) >= EVERY {
            // if c % (EVERY-2) == 0 {
            if c % 2 == 0 {
            // while pairing.count_children() > 0 && pairing.count_children() % EVERY == 0 {
                let (new_pairing, item, newly_corrupted) = pairing.delete_min();
                all_corrupted += newly_corrupted.len();
                non_corrupted_pops += usize::from(item.is_some());
                pairing = new_pairing;
            }
            // }
        }
        let ever_corrupted_fraction = all_corrupted as f64 / c as f64;
        print!(
            "e: {e} n: {c}\tcurrent_corrupted: {:.2}\tsize: {}\tsize prop: {:.2}%\tCorrupted fraction (intermediate): {:.2}%\t\t\t",
            pairing.count_corrupted() as f64 / c as f64,
            pairing.size,
            pairing.size as f64 / c as f64 * 100.0,
            ever_corrupted_fraction * 100.0,
        );
        while !pairing.is_empty() {
            // c += 1;
            let (new_pairing, item, newly_corrupted) = pairing.delete_min();
            all_corrupted += newly_corrupted.len();
            non_corrupted_pops += usize::from(item.is_some());
            pairing = new_pairing;
            // if !newly_corrupted.is_empty() {
            //     print!(
            //         "{c}\tTotal corrupted: {all_corrupted}\tNewly corrupted: {}\t",
            //         newly_corrupted.len()
            //     );
            //     let corrupted_fraction = all_corrupted as f64 / n as f64;
            //     println!("Corrupted fraction: {:.2}%", corrupted_fraction * 100.0);
            // }
            // println!(
            //     "Total corrupted: {all_corrupted}\tUncorrupted: {}\tCorrupted: {}",
            //     pairing.count_uncorrupted(),
            //     pairing.count_corrupted()
            // );
        }
        let ever_corrupted_fraction = all_corrupted as f64 / c as f64;
        println!(
            "Corrupted fraction: {:.2}%\tn: {c}\tcheck_sum: {}",
            ever_corrupted_fraction * 100.0,
            (all_corrupted as i64) + (non_corrupted_pops as i64) - (c as i64)
        );
    }
    // println!(
    //     "Total corrupted: {all_corrupted}\tUncorrupted: {}\tCorrupted: {}",
    //     pairing.count_uncorrupted(),
    //     pairing.count_corrupted()
    // );
}

pub fn interleave1<const EVERY: usize>() -> f64 {
    // let n = 10_000_000;
    // const EVERY: usize = 15;
    // println!("EVERY: {EVERY}");
    let e: usize = 22;

    let n = 1 << e;

    let mut pairing: SoftHeap<EVERY, _> = SoftHeap::default();

    let mut all_corrupted = 0;
    let mut _non_corrupted_pops = 0; // Changed to _non_corrupted_pops
    let mut c = 0;
    for _i in 0..n {
        c += 1;
        pairing = pairing.insert(c);
        // if pairing.root.as_ref().map(|r| r.children.len()).unwrap_or(0) >= EVERY {
        // if c % (EVERY-2) == 0 {
        if c % 2 == 0 {
        // while pairing.count_children() > EVERY && pairing.count_children() % EVERY == 0 {
        // while pairing.count_children() > EVERY {
            let (new_pairing, item, newly_corrupted) = pairing.delete_min();
            all_corrupted += newly_corrupted.len();
            _non_corrupted_pops += usize::from(item.is_some()); // Changed to _non_corrupted_pops
            pairing = new_pairing;
        }
    }
    while !pairing.is_empty() {
        let (new_pairing, item, newly_corrupted) = pairing.delete_min();
        all_corrupted += newly_corrupted.len();
        _non_corrupted_pops += usize::from(item.is_some()); // Changed to _non_corrupted_pops
        pairing = new_pairing;
    }
    all_corrupted as f64 / c as f64
}

// The run_for_range macro is no longer needed and can be removed.

pub fn sort1<const EVERY: usize>() -> (f64, f64) {
    // let n = 10_000_000;
    // const EVERY: usize = 15;
    // println!("EVERY: {EVERY}");
    let e: usize = 23;

    let n = 1 << e;
    let mut max_corrupted = 0;

    let mut pairing: SoftHeap<EVERY, _> = SoftHeap::default();

    let mut all_corrupted = 0;
    let mut _non_corrupted_pops = 0; // Changed to _non_corrupted_pops
    let mut x = (0..n).collect::<Vec<_>>();
    x.shuffle(&mut rand::rng());
    for i in x {
        pairing = pairing.insert(i);
        max_corrupted = max(max_corrupted, pairing.count_corrupted());
    }
    while !pairing.is_empty() {
        let (new_pairing, item, newly_corrupted) = pairing.delete_min();
        all_corrupted += newly_corrupted.len();
        _non_corrupted_pops += usize::from(item.is_some()); // Changed to _non_corrupted_pops
        pairing = new_pairing;
        max_corrupted = max(max_corrupted, pairing.count_corrupted());
    }
    (all_corrupted as f64 / n as f64, max_corrupted as f64 / n as f64)
}

pub fn interleave_n() {
    println!(" N\tCorrupted fraction\tlog2(1/N)\tlog2(N)\tN*result");
    seq!(N in 2..=256 {
        let result = interleave1::<N>();
        let lresult = -result.log2();
        let l_n = (N as f64).log2();
        let x = N as f64 * result;
        println!("{:2}\t\t{:6.2}%\t{lresult:6.2}\t{l_n:6.2}\t{x:6.2}", N, result * 100.0);
    });
}

pub fn sort_n() {
    println!("Sort N");
    println!(" N\tCorrupted fraction\tlog2(1/N)\tlog2(N)\tN*result\tmax_frac\tmax_frac*N");
    seq!(N in 2..=256 {
        let (result, max_frac) = sort1::<N>();
        let lresult = -result.log2();
        let l_n = (N as f64).log2();
        let x = N as f64 * result;
        let xx = max_frac * N as f64;
        println!("{:2}\t\t{:6.2}%\t{lresult:6.2}\t{l_n:6.2}\t{x:6.2}\t{:6.2}\t{xx:6.2}", N, result * 100.0, max_frac * 100.0);
    });
}

pub fn main() {
    one_batch();
    // interleave();
    // interleave_n();
    // sort_n();
}
