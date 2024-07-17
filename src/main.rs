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


fn main() {
    let random_bits = generate_random_bits(20_000);

    println!("{:?}", test1(&random_bits));
    println!("{:?}", test2(&random_bits));
    println!("{:?}", test3(&random_bits));
    /*for byte4 in &random_bits {
           print!("\n{:032b}", byte4);
    }*/
}
