use threshold_secret_sharing::shamir::ShamirSecretSharing;

fn main() {

    let tss = ShamirSecretSharing {
        threshold: 1,
        share_count: 20,
        prime: 7757,
    };

    let random_number = 71;
    let r_shares = tss.share(random_number);

    let r_bits : Vec<i64> = (0..63)
        .map(|n| if random_number & 1<<n != 0 {
            1
        }
        else {
            0
        }
        ).collect();

    let r_bit_shares: Vec<Vec<i64>> = r_bits
    .iter()
    .map(|b| {
        tss.share(*b)
    })
    .collect();

    let target_number = 150;
    let x_shares = tss.share(target_number);

    let c0 = r_shares[0] + x_shares[0];
    let c1 = r_shares[1] + x_shares[1];
    let real_c = random_number + target_number;
    let revealed_c = tss.reconstruct(&[0,1], &[c0, c1]);

    println!("Real C: {} Revealed C: {}", real_c, revealed_c);

    // Now, we split C into its bits, and perform the protocol with the bits of the shared random
    // value

    let c_bits: Vec<i64> = (0..63)
    .map(|n| if revealed_c & 1<<n != 0 {
        1
    }
    else {
        0
    }).collect();

    let mut t: Vec<i64>;
    let mut x_bit_shares = Vec::<Vec<i64>>::new();

    if c_bits[0] == 0 {
        x_bit_shares.push(vec![
            r_bit_shares[0][0],
            r_bit_shares[0][1],
            r_bit_shares[0][2],
            r_bit_shares[0][3],
            r_bit_shares[0][4],
            r_bit_shares[0][5],
            r_bit_shares[0][6],
            r_bit_shares[0][7],
        ]);
        t = vec![
            r_bit_shares[0][0],
            r_bit_shares[0][1],
            r_bit_shares[0][2],
            r_bit_shares[0][3],
            r_bit_shares[0][4],
            r_bit_shares[0][5],
            r_bit_shares[0][6],
            r_bit_shares[0][7],
        ];
    }
    else if c_bits[0] == 1 {
        /*let compliment0 = (1 - r_bit_shares[0][0])%tss.prime;
        let compliment1 = (1 - r_bit_shares[0][1])%tss.prime;*/
        let compliment = vec![
            (1-r_bit_shares[0][0]+tss.prime)%tss.prime,
            (1-r_bit_shares[0][1]+tss.prime)%tss.prime,
            (1-r_bit_shares[0][2]+tss.prime)%tss.prime,
            (1-r_bit_shares[0][3]+tss.prime)%tss.prime,
            (1-r_bit_shares[0][4]+tss.prime)%tss.prime,
            (1-r_bit_shares[0][5]+tss.prime)%tss.prime,
            (1-r_bit_shares[0][6]+tss.prime)%tss.prime,
            (1-r_bit_shares[0][7]+tss.prime)%tss.prime
        ];
        x_bit_shares.push(compliment);
        let zero_shares = tss.share(0);
        t = vec![
            zero_shares[0],
            zero_shares[1],
            zero_shares[2],
            zero_shares[3],
            zero_shares[4],
            zero_shares[5],
            zero_shares[6],
            zero_shares[7]
        ];
    }
    else {
        panic!("c[0] wasn't equal to zero or one");
    }

    //println!("x bit_shares: {:?}", x_bit_shares);
    //reconsturct x bit zero
    //println!("reconstructed: {}", tss.reconstruct(&[0,1, 2, 3], &x_bit_shares[0]));


for i in 1..63 {
        let a = vec![
            (r_bit_shares[i][0] * t[0])%tss.prime,
            (r_bit_shares[i][1] * t[1])%tss.prime,
            (r_bit_shares[i][2] * t[2])%tss.prime,
            (r_bit_shares[i][3] * t[3])%tss.prime,
            (r_bit_shares[i][4] * t[4])%tss.prime,
            (r_bit_shares[i][5] * t[5])%tss.prime,
            (r_bit_shares[i][6] * t[6])%tss.prime,
            (r_bit_shares[i][7] * t[7])%tss.prime,
        ];
        let a2 = vec![
            (2*a[0]+tss.prime)%tss.prime,
            (2*a[1]+tss.prime)%tss.prime,
            (2*a[2]+tss.prime)%tss.prime,
            (2*a[3]+tss.prime)%tss.prime,
            (2*a[4]+tss.prime)%tss.prime,
            (2*a[5]+tss.prime)%tss.prime,
            (2*a[6]+tss.prime)%tss.prime,
            (2*a[7]+tss.prime)%tss.prime
        ];
        if c_bits[i] == 0 {
            let ri_plus_t = vec![
                r_bit_shares[i][0] + t[0],
                r_bit_shares[i][1] + t[1],
                r_bit_shares[i][2] + t[2],
                r_bit_shares[i][3] + t[3],
                r_bit_shares[i][4] + t[4],
                r_bit_shares[i][5] + t[5],
                r_bit_shares[i][6] + t[6],
                r_bit_shares[i][7] + t[7],
            ];
            x_bit_shares.push(vec![
                (ri_plus_t[0] - a2[0]+tss.prime)%tss.prime,
                (ri_plus_t[1] - a2[1]+tss.prime)%tss.prime,
                (ri_plus_t[2] - a2[2]+tss.prime)%tss.prime,
                (ri_plus_t[3] - a2[3]+tss.prime)%tss.prime,
                (ri_plus_t[4] - a2[4]+tss.prime)%tss.prime,
                (ri_plus_t[5] - a2[5]+tss.prime)%tss.prime,
                (ri_plus_t[6] - a2[6]+tss.prime)%tss.prime,
                (ri_plus_t[7] - a2[7]+tss.prime)%tss.prime,
            ]);
            t = vec![
                (ri_plus_t[0] - a[0]+tss.prime)%tss.prime,
                (ri_plus_t[1] - a[1]+tss.prime)%tss.prime,
                (ri_plus_t[2] - a[2]+tss.prime)%tss.prime,
                (ri_plus_t[3] - a[3]+tss.prime)%tss.prime,
                (ri_plus_t[4] - a[4]+tss.prime)%tss.prime,
                (ri_plus_t[5] - a[5]+tss.prime)%tss.prime,
                (ri_plus_t[6] - a[6]+tss.prime)%tss.prime,
                (ri_plus_t[7] - a[7]+tss.prime)%tss.prime,
            ];
        }
        else if c_bits[i] == 1 {
            let compliment = vec![
                (1-r_bit_shares[i][0]+tss.prime)%tss.prime,
                (1-r_bit_shares[i][1]+tss.prime)%tss.prime,
                (1-r_bit_shares[i][2]+tss.prime)%tss.prime,
                (1-r_bit_shares[i][3]+tss.prime)%tss.prime,
                (1-r_bit_shares[i][4]+tss.prime)%tss.prime,
                (1-r_bit_shares[i][5]+tss.prime)%tss.prime,
                (1-r_bit_shares[i][6]+tss.prime)%tss.prime,
                (1-r_bit_shares[i][7]+tss.prime)%tss.prime
            ];
            let compliment_minus_t = vec![
                (compliment[0]-t[0]+tss.prime)%tss.prime,
                (compliment[1]-t[1]+tss.prime)%tss.prime,
                (compliment[2]-t[2]+tss.prime)%tss.prime,
                (compliment[3]-t[3]+tss.prime)%tss.prime,
                (compliment[4]-t[4]+tss.prime)%tss.prime,
                (compliment[5]-t[5]+tss.prime)%tss.prime,
                (compliment[6]-t[6]+tss.prime)%tss.prime,
                (compliment[7]-t[7]+tss.prime)%tss.prime,
            ];
            x_bit_shares.push(vec![
                (compliment_minus_t[0] - a2[0]+tss.prime)%tss.prime,
                (compliment_minus_t[1] - a2[1]+tss.prime)%tss.prime,
                (compliment_minus_t[2] - a2[2]+tss.prime)%tss.prime,
                (compliment_minus_t[3] - a2[3]+tss.prime)%tss.prime,
                (compliment_minus_t[4] - a2[4]+tss.prime)%tss.prime,
                (compliment_minus_t[5] - a2[5]+tss.prime)%tss.prime,
                (compliment_minus_t[6] - a2[6]+tss.prime)%tss.prime,
                (compliment_minus_t[7] - a2[7]+tss.prime)%tss.prime
            ]);
        }
        else {
            panic!("found a value besides 0 or 1 for a c bit");
        }
    }

    /*x_bit_shares.iter().enumerate().for_each(|(i, x)| {
        //println!("Row {}: {:?}", i, x);
        println!("c val:{} {}", c_bits[i], tss.reconstruct(&[0,1,2,3,4,5,6,7], x));
    });*/

    x_bit_shares
        .iter()
        .for_each(|shares| {
            println!("{}", tss.reconstruct(&[0,1,2,3,4,5,6,7], shares));
        });

}
