use std::ops::Index;
use std::ops::IndexMut;
use std::process;

#[derive(Debug)]
struct Board {
    cols: usize,
    rows: usize,
    data: Vec<u32>,
    marked_mask: Vec<bool>,
    won: bool
}

impl Board {
    fn new_empty() -> Board { //cols: u8, rows: u8) -> Board {
        Board {
            cols: 5,
            rows: 5,
            data: vec![0; 25],
            marked_mask: vec![false; 25],
            won: false
        }
    }
    fn set_data(&mut self, data: Vec<u32>) { //cols: u8, rows: u8) -> Board {
        self.data = data;
    }
    fn mark_draw(&mut self, draw: u32){
        for (i, num) in self.data.iter().enumerate() {
            if *num == draw {
                self.marked_mask[i] = true;
            }
        }
    }
    fn sum_unmarked(&mut self) -> u32 {
        let mut sum = 0;
        for (i, is_marked) in self.marked_mask.iter().enumerate() {
            if !is_marked {
                sum += *self.data.get(i).unwrap();
            }
        }
        sum
    }
    fn check_win(&mut self) -> bool {
        for i in 0..5 {
            let mut everything_true: bool = true;
            for j in 0..5 {
                if !self.get_mask(i, j) {
                    everything_true = false;
                }
            }
            if everything_true {
                return true
            }
        }
        for i in 0..5 {
            let mut everything_true: bool = true;
            for j in 0..5 {
                if !self.get_mask(j, i) {
                    everything_true = false;
                }
            }
            if everything_true {
                return true
            }
        }
        false
    }
    fn get(&self, c: usize, r: usize) -> u32 {
        *self.data.get(r * self.rows + c).unwrap()
    }
    fn get_mask(&self, c: usize, r: usize) -> bool {
        *self.marked_mask.get(r * self.rows + c).unwrap()
    }
}

impl Index<usize> for Board {
    type Output = [u32];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols .. (index+1) * self.cols]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        println!("Accessing {:?}-side of balance mutably", index);
        &mut self.data[index * self.cols .. (index+1) * self.cols]
    }
}

fn read_boards(lines: &mut std::str::Lines<'_>) -> Vec<Board> {
    let mut res_vec: Vec<Board> = Vec::new();
    loop {
        match lines.next() {
            Some(x) => {
                if x == "" {
                    continue
                } else {
                    let mut data: Vec<u32> = Vec::new();
                    let mut vec = x.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                    data.append(&mut vec);
                    for _ in 0..4 {
                        let elem = lines.next().unwrap();
                        let mut vec = elem.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                        data.append(&mut vec);
                    }
                    let mut b = Board::new_empty();
                    b.set_data(data);
                    res_vec.push(b);
                }

            },
            None => { break }
        }
    }
    res_vec
}

fn read_draws(line: &str) -> Vec<u32> {
    line.split(',').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}

fn main() {
    let inp = std::fs::read_to_string("in.txt").expect("couldn't load file.");
    let mut iter = inp.lines();

    let draws: Vec<u32> = read_draws(iter.next().unwrap());
    println!("draws: {:?}", draws);

    let mut boards: Vec<Board> = read_boards(&mut iter);
    println!("boards: {:?}", boards);

    for draw in draws {

        for board in &mut boards {
            if board.won {
                continue
            }
             board.mark_draw(draw);
             if board.check_win() {
                println!("won {:?}", board);
                board.won = true;
                println!("sum {:?}", board.sum_unmarked() * draw);
             }
        }
    }
}
