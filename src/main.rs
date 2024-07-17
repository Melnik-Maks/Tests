use rand::Rng;

fn generate_random_bits(length: usize) -> Vec<u32> {
    let byte_length = length / 32; // кількість байтів для зберігання length бітів
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u32; byte_length];
    rng.fill(&mut bytes[..]);
    bytes
}

fn test1(bits: &[u32]) -> bool {
    let mut ones = 0;
    for byte4 in bits {
        for i in 0..32 {
            if byte4 & (1 << i) != 0 {
                ones += 1;
            }
        }
    }
    println!("{}", ones);
    9654 <= ones && ones <= 10346
}


fn test2(bits: &[u32]) -> bool {
    let mut max_zeros = 0;
    let mut max_ones = 0;
    let mut current_zeros = 0;
    let mut current_ones = 0;

    for byte4 in bits {
        for i in 0..32 {
            if byte4 & (1 << i) != 0 {
                current_ones += 1;
                max_zeros = max_zeros.max(current_zeros);
                current_zeros = 0;
            } else {
                current_zeros += 1;
                max_ones = max_ones.max(current_ones);
                current_ones = 0;
            }
        }
    }

    max_zeros = max_zeros.max(current_zeros);
    max_ones = max_ones.max(current_ones);
    println!("Max: {}, {}", max_ones, max_zeros);

    max_zeros <= 36 && max_ones <= 36

}


fn test3(bits: &[u32]) -> bool {
    let mut mas = vec![0u32; 16];

    for byte4 in bits {
        for i in 0..8 {
            mas[((byte4 << 4*i) >> 28)as usize] += 1;
            //print!("{} ",(byte4 << 4*i) >> 28);
        }
        //println!("\n{:?}", mas);
    }
    let mut sum = 0f32;
    for x in mas.iter() {
        sum += (x * x) as f32;
    }
    //println!("{}", sum);
    let x3:f32 = (16.0 / (8.0 * bits.len() as f32)) * sum - (8 * bits.len()) as f32;
    println!("{}", x3);
    1.03 <= x3 && x3 <= 57.4
}

fn test4(bits: &[u32]) -> bool {
    let mut mas_zeros = [0u32; 6];
    let mut mas_ones = [0u32; 6];

    let mut current_zeros = 0;
    let mut current_ones = 0;

    for byte4 in bits {
        for i in 0..32 {
            if byte4 & (1 << i) != 0 {
                current_ones += 1;
                if current_zeros > 0 {
                    mas_zeros[if current_zeros > 5 {5} else {current_zeros - 1}] += 1;
                }
                current_zeros = 0;
            } else {
                current_zeros += 1;
                if current_ones > 0 {
                    mas_ones[if current_ones > 5 {5} else {current_ones - 1}] += 1;
                }
                current_ones = 0;
            }
        }
    }
    if current_ones > 0 {
        mas_ones[if current_ones > 5 {5} else {current_ones - 1}] += 1;
    }
    if current_zeros > 0 {
        mas_zeros[if current_zeros > 5 {5} else {current_zeros - 1}] += 1;
    }
    println!("{:?}, {:?}", mas_ones, mas_zeros);
    if  !(2267 <= mas_ones[0] && mas_ones[0] <= 2733) ||
        !(2267 <= mas_zeros[0] && mas_zeros[0] <= 2733) ||
        !(1079 <= mas_ones[1] && mas_ones[1] <= 1421) ||
        !(1079 <= mas_zeros[1] && mas_zeros[1] <= 1421) ||
        !(502 <= mas_ones[2] && mas_ones[2] <= 748) ||
        !(502 <= mas_zeros[2] && mas_zeros[2] <= 748) ||
        !(223 <= mas_ones[3] && mas_ones[3] <= 402) ||
        !(223 <= mas_zeros[3] && mas_zeros[3] <= 402) ||
        !(90 <= mas_ones[4] && mas_ones[4] <= 223) ||
        !(90 <= mas_zeros[4] && mas_zeros[4] <= 223) ||
        !(90 <= mas_ones[5] && mas_ones[5] <= 223) ||
        !(90 <= mas_zeros[5] && mas_zeros[5] <= 223) {
        return false;
    }
    true

}

fn main() {
    let random_bits = generate_random_bits(20_000);

    println!("{:?}", test1(&random_bits));
    println!("{:?}", test2(&random_bits));
    println!("{:?}", test3(&random_bits));
    println!("{:?}", test4(&random_bits));

    /*for byte4 in &random_bits {
           print!("\n{:032b}", byte4);
    }*/
}
