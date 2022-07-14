pub fn first_to_last(input: Vec<usize>) -> Vec<usize> {
    let mut output = input.clone();
    let temp_first = output[0];
    output.push(temp_first);
    output.remove(0);

    output
}

// example0 [1, 2, 3] -> [2 3 1]
// example1 [1, 2, 3, 4, 5] -> [4, 5, 2, 3, 1]
// example2 [1, 2, 3, 4, 5, 6] -> [5, 6, 4, 3, 1, 2]
pub fn reverb_by_pair(input: Vec<usize>) -> Vec<usize> {
    if input.len() <= 3 {
         return first_to_last(input);
    }
    let mut offset: usize = 1;
    let last_index: usize = input.len() - 1;
    let mut output: Vec<usize> = Vec::new();

    for i in 0..input.len() {
        let push_index: usize = last_index - offset;
        output.push(input[push_index].clone());

        if i % 2 == 0 {
            offset -= 1;
        } else {
            offset += 3;
        }
    }

    //swap middle pair if exists
    if output.len() % 2 == 0 && output.len() % 4 != 0 {
        let left = (output.len() as f64 / 4.0).floor() as usize;
        let right = (output.len() as f64 / 4.0).ceil() as usize;
        output.swap(left,right);
    }

    return output;
}
