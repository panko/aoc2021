fn main() {
    let data = std::fs::read_to_string("in.txt").expect("couldn't load file.");
    let vec: Vec<&str> = data.lines().collect();

    fn get_freqs_one(asd: &Vec<&str>) -> Vec<u32>{
        let mut zero_freq = vec![0; 12];
        let mut one_freq = vec![0; 12];
        let mut res_one = vec![0; 12];

        for line in asd {
            for (i, character) in line.chars().enumerate() {
                match character {
                    '0' => zero_freq[i] += 1,
                    '1' => one_freq[i] += 1,
                    _ => panic!("unexpected")
                }
            }
        }
        for i in 0..12 {
            if one_freq[i] == zero_freq[i]  {
                res_one[i] = 1
            }
            else if one_freq[i] > zero_freq[i] {
                res_one[i] = 1
            }
            else if one_freq[i] < zero_freq[i] {
                res_one[i] = 0
            }
        }
        res_one
    }

    fn get_freqs_zero(asd: &Vec<&str>) -> Vec<u32>{
        let mut zero_freq = vec![0; 12];
        let mut one_freq = vec![0; 12];
        let mut res_zero = vec![0; 12];
        for line in asd {
            for (i, character) in line.chars().enumerate() {
                match character {
                    '0' => zero_freq[i] += 1,
                    '1' => one_freq[i] += 1,
                    _ => panic!("unexpected")
                }
            }
        }
        for i in 0..12 {
            if one_freq[i] == zero_freq[i]  {
                res_zero[i] = 0
            }
            else if one_freq[i] > zero_freq[i] {
                res_zero[i] = 0
            }
            else if one_freq[i] < zero_freq[i] {
                res_zero[i] = 1
            }
        }
        res_zero
    }

    let mut work_vec = vec.clone();
    for i in 0..12 {
        let is_one_the_most_freq_vec: Vec<u32> = get_freqs_one(&work_vec);

        work_vec = work_vec.iter()
                        .filter(|s| s.chars().nth(i).unwrap().to_digit(10).unwrap() == *is_one_the_most_freq_vec.get(i).unwrap())
                        .cloned()
                        .collect();

        match work_vec.len() == 1 {
            true => break,
            false => continue
        }

    }
    let oxygen_generator_rating = isize::from_str_radix(&work_vec[0], 2).unwrap();

    let mut work_vec: Vec<&str> = vec.clone();
    for i in 0..12 {
        let is_zero_the_most_freq_vec: Vec<u32> = get_freqs_zero(&work_vec);

        work_vec = work_vec.iter()
                .filter(|s| s.chars().nth(i).unwrap().to_digit(10).unwrap() == *is_zero_the_most_freq_vec.get(i).unwrap())
                .cloned()
                .collect();

        match work_vec.len() == 1 {
            true => break,
            false => continue
        }
    }
    let co2_scrubber_rating = isize::from_str_radix(&work_vec[0], 2).unwrap();

    println!("{:?}", oxygen_generator_rating * co2_scrubber_rating);
}
