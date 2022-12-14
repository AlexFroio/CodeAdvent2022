use common_utils::open_file;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    /// Elf type
    /// Keeps track of which elf and how many calories they have.
    /// Unnecessary ultimately, but I didn't know what type of wrench part 2 would throw.
    num:u32,
    total_cal:u64,
}

impl Default for Elf {
    /// Add Default trait to Elf, since we need a clean object for pushing.
    fn default() -> Self {
        ///Just return an elf assuming it's the first one, and with 0 calories.
        Elf{num:1, total_cal:0}
    }
}


fn sum_cal(in_str:String) -> Vec<Elf>{
    /// Given an entire input file, 
    /// with a blank line delimiting each elf
    /// add the numbers together per elf.
    let mut elves:Vec<Elf> = Vec::new();
    elves.push(Elf::default());
    for line in in_str.lines() {
        if line != "" {
            elves.last_mut().unwrap().total_cal += line.parse::<u64>().unwrap()
        }
        else {
            elves.push(Elf{num: elves.len() as u32 + 1, total_cal: 0});
        }
    }
    return elves;
}

fn main() {
    // Windows was used to run this code so it, regrettably, uses NT style paths.
    let contents = open_file("day01\\src\\input.txt");

    if let Ok(foo) = contents {
        let mut elves = sum_cal(foo);
        elves.sort_by(|a, b| b.total_cal.cmp(&a.total_cal));

        for elf in elves.clone() {
            println!("{}, {}", elf.num, elf.total_cal);
        }
        let top_3_sum = elves[0].total_cal + elves[1].total_cal + elves[2].total_cal;
        println!("{}", top_3_sum);
    }
    else{
        println!("You're an idiot");

    }

}
