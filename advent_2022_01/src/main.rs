use std::fs;

fn main() {
    // let file_name = "sample.txt";
    let file_name = "input.txt";
    let file_contents = fs::read_to_string(file_name).expect("File has contents");
    let elves = file_contents.split("\n\n");

    const AMOUNT_TOP_ELVES: usize = 3;
    let mut leading_elves = [0; AMOUNT_TOP_ELVES];

    for elf in elves {
        let total_cal_for_elf = get_total_cal_for_elf(elf);
        for (place, current_leading_elf) in leading_elves.into_iter().enumerate() {
            if total_cal_for_elf > current_leading_elf {
                insert_into_array_and_drop_last(&mut leading_elves, place, total_cal_for_elf);
                break;
            }
        }
    }

    let sum_of_leading_elves = leading_elves.into_iter().reduce(|a, b| a + b).unwrap();
    println!("Sum of leading elves is {sum_of_leading_elves}")
}

// ref: string pointer: https://h2co3.github.io/pattern/
fn get_total_cal_for_elf(elf: &str) -> i32 {
    let foods = elf.split("\n");
    let total_cal_for_elf = foods.fold(0, |total, food_str| match food_str.parse::<i32>() {
        Ok(food) => total + food,
        _ => total,
    });
    total_cal_for_elf
}

// question: &mut? Don't know what that is
fn insert_into_array_and_drop_last<T>(array: &mut [T], index: usize, value: T) {
    // ref: learning about arrays + inserts https://stackoverflow.com/a/69940015
    array[index..].rotate_right(1);
    array[index] = value;
}
