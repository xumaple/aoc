use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sum = 0;
    for l in read_lines("src/02/input.txt")? {
        let l = l?;
        let (_game_num_str, hands) = l.split_once(": ").expect("Did not find ':' in game line");
        // let game_num: i32 = unsafe { _game_num_str.get_unchecked(5..) }.parse()?;

        let mut maxes: HashMap<&str, i32> = HashMap::new();

        let mut on_num = true;
        let mut prev_num: i32 = -1;
        for token in hands.split(" ") {
            match on_num {
                true => {
                    prev_num = token.parse::<i32>().expect(format!("Unable to parse int: {token}").as_str());
                    on_num = false;
                }
                false => {
                    let curr_val = maxes.entry(&token.trim_end_matches(&[',', ';'])).or_insert(0);
                    if prev_num > *curr_val {
                        *curr_val = prev_num;
                    }

                    on_num = true;
                }
            }
        }
        sum += maxes.values().fold(1, |a, &b| a * b);
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
