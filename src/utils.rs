use std::fs;
use std::io::{BufReader, BufRead};

//read input of day number "day" and return a String (with breaklines)
pub fn read_input_string(day: u8) -> String{

    let path = format!("input/day_{}_input.txt", day);
    
    let input_of_the_day  = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Unable to retreive inputs from {}, error: {}", path, err));

    input_of_the_day

}

//read input of day number "day" and return vector of String, one string per line
pub fn read_input_of_the_day_lines(day: u8) -> Vec<String>{
    
    let path = format!("input/day_{}_input.txt", day);

    //Open file
    let file = fs::File::open(&path)
        .unwrap_or_else(|err| panic!("Unable to retreive inputs from {}, error: {}", path, err));

    let reader = BufReader::new(file);

    reader.lines()
          .map(|l| l.unwrap())
          .collect()
}  

//take a reference to an array containing strings and return the array of u32, panic if not possible
pub fn vec_string_to_int(vec_string: &Vec<String>) -> Vec<u32>{
    //iterate over the words, map a string to an int (panic if not possible), and collect
    vec_string.iter()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()

} 


