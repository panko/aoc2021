fn main() {
    let data = std::fs::read_to_string("in.txt").expect("couldn't load file.");
    let vec: Vec<&str> = data.lines().collect();

    let mut zero_freq = vec![0; 12];
    let mut one_freq = vec![0; 12];

    for line in vec {
        for (i, character) in line.chars().enumerate() {
            match character {
                '0' => zero_freq[i] += 1,
                '1' => one_freq[i] += 1,
                _ => panic!("unexpected")
            }
        }
    }
    let bin_one: String = one_freq
        .iter()
        .map(|ch| std::char::from_digit((ch > &500) as u32, 10).unwrap())
        .collect::<String>();

    let bin_zero: String = zero_freq
        .iter()
        .map(|ch| std::char::from_digit((ch > &500) as u32, 10).unwrap())
        .collect::<String>();
    
    let gamma_rate = isize::from_str_radix(&bin_one, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&bin_zero, 2).unwrap();
    println!("{:?}", gamma_rate * epsilon_rate);
}
