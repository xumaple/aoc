use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maxes: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    let mut sum = 0;
    for l in read_lines("src/02/input.txt")? {
        let l = l?;
        let (game_num_str, hands) = l.split_once(": ").expect("Did not find ':' in game line");
        let game_num: i32 = unsafe { game_num_str.get_unchecked(5..) }.parse()?;

        let mut on_num = true;
        let mut prev_num: i32 = -1;
        let mut game_success = true;
        for token in hands.split(" ") {
            match on_num {
                true => {
                    prev_num = token.parse::<i32>().expect(format!("Unable to parse int: {token}").as_str());
                    on_num = false;
                }
                false => {
                    if prev_num > *maxes.get(&token.trim_end_matches(&[',', ';'])).expect(format!("Unable to identify color {token}").as_str()) {
                        game_success = false;
                        break;
                    }
                    on_num = true;
                }
            }
        }
        if game_success {
            sum += game_num;
        }
    }
    println!("{sum}");
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
