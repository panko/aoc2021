fn main() {
    let data = std::fs::read_to_string("in.txt").expect("couldn't load file.");
    let mut num: u32 = 0;
    let mut res: u32 = 0;
    for elem in data.lines() {
        let new_num: u32 = elem.parse().unwrap();
        if num == 0 {
            num = new_num;
            continue;
        } 
        if new_num > num {
            res += 1;
        }
        num = new_num;
    } 
    println!("{}", res);
}
