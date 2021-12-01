fn main() {
    let data = std::fs::read_to_string("in.txt").expect("couldn't load file.");
    let mut before_last: u32 = 0;
    let mut last: u32 = 0;
    let mut num: u32 = 0;
    let mut res: u32 = 0;
    for (i, elem) in data.lines().enumerate() {
        let new_num: u32 = elem.parse().unwrap();

        if i == 0 {
            before_last = new_num;
            continue;
        }

        if i == 1 {
            last = new_num;
            continue;
        }

        if i == 2 {
            num = before_last + last + new_num;
            continue;
        }

        let sum = before_last + last + new_num

        if sum > num {
            res += 1;
        }

        before_last = last;
        last = new_num;
        num = sum;
    } 
    println!("{}", res);
}
