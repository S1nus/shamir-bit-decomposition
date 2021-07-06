use threshold_secret_sharing::shamir::ShamirSecretSharing;

fn main() {

    let tss = ShamirSecretSharing {
        threshold: 1,
        share_count: 80,
        prime: 7757,
    };

    let mut indices = [0; 80];
    for i in (0..80) {
        indices[i] = i;
    }

    let r = 250; // 1101
    let x = 3000; // 0010

    let r_bits: Vec<i64> = (0..16)
        .map(|n| if r & 1<<n !=0 { 1 } else { 0 })
        .collect();

    let r_bit_shares: Vec<Vec<i64>> = r_bits
        .iter()
        .map(|b| tss.share(*b))
        .collect();

    let r_shares = tss.share(r);

    let x_shares = tss.share(x);
    
    let c_shares: Vec<i64> = r_shares
        .iter()
        .zip(x_shares.iter())
        .map(|(r, x)| {
            r + x
        })
        .collect();

    let real_c = x+r;
    let revealed_c = tss.reconstruct(&[0,1], &[c_shares[0], c_shares[1]]);

    let c_bits: Vec<i64> = (0..16)
        .map(|n| if real_c & 1<<n != 0 { 1 } else { 0 })
        .collect();


    let mut x_bit_shares : Vec<Vec<i64>> = Vec::new();
    let mut t : Vec<i64> = Vec::new();

    println!("R[0] = {}", r_bits[0]);
    if c_bits[0] == 0 {
        println!("c[0] == 0, so setting x[0] to r[0], and t to r[0]");
        x_bit_shares.push(r_bit_shares[0].clone());
        t = r_bit_shares[0].clone();
    }
    else if c_bits[0] == 1 {
        println!("c[0] == 1, so setting x[0] to 1 - r[0], and t to 0");
        let compliment: Vec<i64> = r_bit_shares[0]
            .iter()
            .map(|s| (1 - s + tss.prime)%tss.prime )
            .collect();
        println!("compliment should equal {}, got {}", 1-r_bits[0], tss.reconstruct(&indices, &compliment));
        x_bit_shares.push(compliment);
        let zero_shares = tss.share(0);
        t = zero_shares;
    }
    else {
        panic!("C_bits[0] wasn't a bit :(");
    }
    println!("x[0] = {:?}", tss.reconstruct(&indices, &x_bit_shares[0]));
    println!("t = {:?}", tss.reconstruct(&indices, &r_bit_shares[0]));

    for i in (1..16) {
        println!("r[{}] = {}",i, r_bits[i]);
        let a: Vec<i64> = r_bit_shares[i]
        .iter()
        .zip(t.iter())
        .map(|(ri, t)| (ri*t)%tss.prime)
        .collect();

        println!("A = R[{}] * t = {}",i, tss.reconstruct(&indices, &a));

        let a2: Vec<i64> = r_bit_shares[i]
        .iter()
        .zip(t.iter())
        .map(|(ri, t)| (2*ri*t)%tss.prime)
        .collect();

        println!("2A = 2 * R[{}] * t = {}",i, tss.reconstruct(&indices, &a2));

        if c_bits[i] == 0 {
            println!("c[{}] is 0",i);
            let xi: Vec<i64> = r_bit_shares[i]
            .iter()
            .zip(t.iter())
            .zip(a2.iter())
            .map(|((ri, t), a2)| (ri + t -a2 + tss.prime)%tss.prime)
            .collect();
            t = r_bit_shares[i]
            .iter()
            .zip(t.iter())
            .zip(a.iter())
            .map(|((ri, t), a)| (ri + t - a + tss.prime)%tss.prime)
            .collect();
            println!("X{} should be {} but is {}",i, tss.reconstruct(&indices, &r_bit_shares[i]) + tss.reconstruct(&indices, &t) - tss.reconstruct(&indices, &a2), tss.reconstruct(&indices, &xi));
            println!("t should be {} but is {}", tss.reconstruct(&indices, &r_bit_shares[i]) + tss.reconstruct(&indices, &t) - tss.reconstruct(&indices, &a), tss.reconstruct(&indices, &t));
            x_bit_shares.push(xi);
        }
        // adapt to i instead of 1
        else if c_bits[i] == 1 {
            println!("c[{}] is 1",i);
            let xi: Vec<i64> = r_bit_shares[i]
            .iter()
            .zip(t.iter())
            .zip(a2.iter())
            .map(|((ri, t), a2)| (1 - ri - t + a2 + tss.prime)%tss.prime)
            .collect();
            t = a.clone();
            println!("X{} should be {} but is {}",i, 1 - tss.reconstruct(&indices, &r_bit_shares[i]) - tss.reconstruct(&indices, &t) + tss.reconstruct(&indices, &a2), tss.reconstruct(&indices, &xi));
            println!("t should be {} but is {}", tss.reconstruct(&indices, &a), tss.reconstruct(&indices, &t));
            x_bit_shares.push(xi);
        }
        else {
            panic!("c[i] not a bit");
        }
    }

    println!("x is: {}", x);
    x_bit_shares
        .iter()
        .for_each(|share| {
            println!("{}", (tss.reconstruct(&indices, &share)+tss.prime)%tss.prime);
        });
}
