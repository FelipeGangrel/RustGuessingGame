use rand::Rng;

pub fn generate_secret_number(max: u32, wrong_number: u32) -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=max);

    if secret_number == wrong_number {
        return generate_secret_number(max, wrong_number);
    }

    return secret_number;
}