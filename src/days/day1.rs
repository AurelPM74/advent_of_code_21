use crate::utils::{self, read_input_of_the_day_lines, vec_string_to_int};   
//use itertools::Itertools;


pub fn solve_1() ->u32{
    //Read the inputs into a vec of strings
    let string_inputs = read_input_of_the_day_lines(1);

    //transform the strings into ints
    let int_inputs = vec_string_to_int(&string_inputs);

    //iterate on a rolling window of size 2 of the vec, create an vec containing a 0 decrease, 
    //1 if increase, and sum the result
    let sum_increase: u32 = int_inputs.windows(2)
                                      .map(|pair| (pair[0]<=pair[1]) as u32)
                                      .sum();
    
    sum_increase
}

pub fn solve_2() ->u32{
    //Read the inputs into a vec of strings
    let string_inputs = read_input_of_the_day_lines(1);

    //transform the strings into ints
    let int_inputs = vec_string_to_int(&string_inputs);

    //Do a rolling windows of size three on the vec, sum the components 
    //Then do a rolling windows of size 2 create an vec containing a 0 decrease, 
    //1 if increase, and sum the result
    let sum_increase:u32 = int_inputs.windows(3)
                                     .map(|triple| triple.iter().sum::<u32>())
                                     .collect::<Vec<_>>()
                                     .windows(2)
                                     .map(|pair| (pair[0]<pair[1]) as u32)
                                     .sum();

    sum_increase
}