use crate::utils::{self, read_input_of_the_day_lines};   
use std::collections::HashMap;

    
//struct that will store the horizontal position and the depth of the sub
struct SubmarinePosition{
    x : i32,
    d : i32,
}

//struct that will store the horizontal position and the depth of the sub
struct SubmarinePositionWithAim{
    x : u32,
    d : u32,
    aim : u32
}
//This struct will have built in method for changing the aim, depth and x position
impl SubmarinePositionWithAim{

    fn down(&mut self, value: u32){
        self.aim = self.aim + value;
    }

    fn up(&mut self, value: u32){
        self.aim = self.aim - value;
    }

    fn forward(&mut self, value: u32){
        self.x = self.x + value;
        self.d = self.d + value * self.aim;
    }

    fn new_instruction(&mut self, instruction: &String, value: u32){
        if instruction == "down"{
            self.down(value)
        } else if instruction == "up"{
            self.up(value)
        } else if instruction == "forward"{
            self.forward(value)
        }
    }
}

pub fn solve_1() -> i32{

    //load the inputs as vec of strings
    let string_inputs = read_input_of_the_day_lines(2);
    
    //let string_inputs = [String::from("forward 5"),
    //                                  String::from("down 5"),
    //                                  String::from("forward 8"),
    //                                  String::from("up 3"),
    //                                  String::from("down 8"),
    //                                  String::from("forward 2")].to_vec();

    //Create a hasmap {instruction, array}, where array will be the array to add to the vector
    let mut instruct_to_arr: HashMap<String, [i32; 2]> = HashMap::new();

    instruct_to_arr.insert(String::from("up"), [0, -1]);
    instruct_to_arr.insert(String::from("down"), [0, 1]);
    instruct_to_arr.insert(String::from("forward"), [1, 0]);

    //We separate the input into instruction and value
    let separated_input:Vec<Vec<String>> = string_inputs.iter()
                                                        .map(|ins|ins.split_whitespace().map(String::from).collect())
                                                        .collect();

    //intitialize submarine position
    let mut sub = SubmarinePosition {
        x : 0,
        d : 0,
    };

    //For each instruction ["instruction", "value"] we change the submarine_position according to 
    //the hashmap instruct_to_arr
    for instruction in separated_input{

        //get the array corresponding to the instruction
        let instruct_array = instruct_to_arr.get(&instruction[0]).unwrap();
        let value: i32 = instruction[1].parse().unwrap();
        
        sub.x = sub.x + instruct_array[0] * value;
        sub.d = sub.d + instruct_array[1] * value;

    }

    //return the depth * the horizontal position
    sub.x * sub.d
}

pub fn solve_2() -> u32{
    //load the inputs as vec of strings
    let string_inputs = read_input_of_the_day_lines(2);

    //Create a hasmap {instruction, array}, where array will be the array to add to the vector

    //We separate the input into instruction and value
    let separated_input:Vec<Vec<String>> = string_inputs.iter()
                                                        .map(|ins|ins.split_whitespace().map(String::from).collect())
                                                        .collect();

    let mut sub = SubmarinePositionWithAim{
        x: 0,
        d: 0,
        aim: 0,
    }; 

    for instruction in separated_input{
        sub.new_instruction(&instruction[0], instruction[1].parse::<u32>().unwrap());   
    }
    sub.x * sub.d
}