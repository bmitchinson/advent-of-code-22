use std::fs;

fn main() {
    // let file_name = "sample.txt";
    let file_name = "input.txt";
    let file_contents = fs::read_to_string(file_name).expect("File has contents");
    let elves = file_contents.split("\n\n");
    // let mut most_cal_elf = 0;

    const AMOUNT_TOP_ELVES: usize = 3;
    let mut leading_elves = [0; AMOUNT_TOP_ELVES];

    for elf in elves {
        let foods = elf.split("\n");
        // refactor: breakout get total_cal_for_elf
        let total_cal_for_elf = foods.fold(0, |total, food_str| match food_str.parse::<i32>() {
            Ok(food) => total + food,
            _ => total,
        });
        // if total_cal_for_elf > most_cal_elf {
        //     most_cal_elf = total_cal_for_elf;
        // }
        // refactor: break out update standings into method
        for (place, current_leading_elf) in leading_elves.into_iter().enumerate() {
            if total_cal_for_elf > current_leading_elf {
                // refactor: break insert into method
                // ref: learning about arrays + inserts https://stackoverflow.com/a/69940015
                leading_elves[place..].rotate_right(1);
                leading_elves[place] = total_cal_for_elf;
                break;
            }
        }
        // if
    }

    // todo: sum top elves
    let sum_of_leading_elves = leading_elves.into_iter().reduce(|a, b| a + b).unwrap();
    println!("Sum of leading elves is {sum_of_leading_elves}")
}
