use std::fs;

fn run_computer(opcodes: &str) -> String {
    let mut i = 0;
    let mut output: Vec<u8> = vec![];
    let ops: Vec<u8> = opcodes
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    while i < ops.len() {
        if i == 0 {
            output.push(2)
        } else {
            // output.push(opcodes.parse::<u8>().unwrap())
            output.push(ops[i])
        }
        i += 1
    }
    let mut output_string: String = output[0..output.len() - 1]
        .into_iter()
        .map(|x| {
            let mut y = x.to_string();
            y.push_str(",");
            y
        })
        .collect();
    output_string.push_str(&output[output.len() - 1].to_string());
    output_string
}

fn main() {
    let contents = fs::read_to_string("./day_2/input.txt").unwrap();
    // run_computer(contents);
    let opcodes: Vec<&str> = contents.split(",").collect();

    println!("{:?}", opcodes);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run_computer() {
        assert_eq!(run_computer("1,0,0,0,99"), "2,0,0,0,99");
    }
}
