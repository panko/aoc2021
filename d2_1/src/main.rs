fn main() {
    let mut distance = 0;
    let mut depth = 0;
    let data = std::fs::read_to_string("in.txt").expect("couldn't load file.");
    let vec: Vec<&str> = data.lines().collect();
    for elem in vec {
        let mut iterator = elem.split_whitespace();
        let direction = iterator.next().unwrap();
        let value: i32 = iterator.next().unwrap().parse().unwrap();

        if direction == "down" {
            depth += value;
        }
        else if direction == "up" {
            depth -= value;
        }
        else if direction == "forward" {
            distance += value;
        }
    }
    println!("{}", distance * depth )
}
