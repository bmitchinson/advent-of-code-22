use std::fs;

fn main() {
    let file_name = "input.txt";
    let file_contents = fs::read_to_string(file_name).expect("File has contents");
    let elves = file_contents.split("\n\n");
    let mut most_cal_elf = 0;

    for elf in elves {
        let foods = elf.split("\n");
        let total_cal_for_elf = foods.fold(0, |total, food_str| match food_str.parse::<i32>() {
            Ok(food) => total + food,
            _ => total,
        });
        if total_cal_for_elf > most_cal_elf {
            most_cal_elf = total_cal_for_elf;
        }
    }

    println!("Top calorie elf is {most_cal_elf}")
}
