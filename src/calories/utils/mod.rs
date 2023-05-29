pub fn parse_portions(read_lines: &String) -> Vec<Vec<isize>> {
    let elf_portions = read_lines.split("\n\n").collect::<Vec<&str>>();
    let mut parsed_portions: Vec<Vec<isize>> = Vec::new();
    for elf_portion in elf_portions {
        let calories = elf_portion.split("\n").collect::<Vec<&str>>();
        let mut parsed_calories: Vec<isize> = Vec::new();
        for calorie in calories {
            match calorie.parse::<isize>() {
                Ok(calorie_value) => {
                    parsed_calories.push(calorie_value);
                }
                Err(_err) => {
                    println!("Failed to parse {}", calorie);
                    parsed_calories.push(0);
                }
            };
        }
        parsed_portions.push(parsed_calories);
    }
    return parsed_portions;
}
