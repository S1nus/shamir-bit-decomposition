use threshold_secret_sharing::shamir::ShamirSecretSharing;

fn main() {

    let tss = ShamirSecretSharing {
        threshold: 1,
        share_count: 20,
        prime: 7757,
    };

    let r = 13; // 1101

    let r_bits: Vec<i64> = (0..4)
        .map(|n| if (r & 1<<n !=0) { 1 } else { 0 })
        .collect();

    println!("r bits: {:?}", r_bits);

    let x = 3; // 0011

}
