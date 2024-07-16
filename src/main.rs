use std::arch::x86_64::_mm_max_ss;
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
    for byte in bits {
        for i in 0..32 {
            if byte & (1 << i) != 0 {
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

    for byte in bits {
        for i in 0..32 {
            if byte & (1 << i) != 0 {
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


fn main() {
    let random_bits = generate_random_bits(20000);

    println!("{:?}", test1(&random_bits));
    println!("{:?}", test2(&random_bits));
    //for byte in &random_bits {
    //        print!("\n{:032b}", byte);
    //}
}
