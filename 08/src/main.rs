use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        panic!("Provide three arguments with width, height path to the input");
    }

    let width = args[1].parse::<usize>().unwrap();
    let height = args[2].parse::<usize>().unwrap();
    println!("W:{}, H:{}", width, height);

    let file = std::fs::File::open(&args[3]).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let file_string = {
        let mut string = String::new();
        reader
            .read_to_string(&mut string)
            .expect("Unable to read file");

        // trim trailing newline
        let len = string.len();
        string.truncate(len - 1);
        string
    };

    let mut layers: Vec<Vec<i32>> = Vec::new();
    let cs = file_string.chars().collect::<Vec<_>>();
    let mut char_iter = cs.iter().peekable();
    while let Some(_) = char_iter.by_ref().peek() {
        let inner = char_iter.by_ref().take(width * height);
        layers.push(inner.map(|x| x.to_digit(10).unwrap() as i32).collect());
    }

    let count_zeros = |x: &Vec<_>| x.iter().filter(|xx| **xx == 0).count();

    let minimal_layer = layers
        .iter()
        .min_by(|x, y| count_zeros(x).cmp(&count_zeros(y)))
        .unwrap();

    let ot = minimal_layer.iter().fold((0, 0), |ones_and_twos, &x| {
        (
            ones_and_twos.0 + ((x == 1) as i32),
            ones_and_twos.1 + ((x == 2) as i32),
        )
    });
    println!("ones {}, twos {}, product {}", ot.0, ot.1, ot.0 * ot.1);

    let mut result_vec = layers.last().unwrap().to_vec();
    for layer in layers.iter().rev().skip(1) {
        result_vec = result_vec
            .iter()
            .zip(layer.iter())
            .map(|(res, new)| if *new == 2 { *res } else { *new })
            .collect();
    }
    for h in 0..height {
        for w in 0..width {
            print!(
                "{}",
                if result_vec[w + h * width] == 0 {
                    "█"
                } else {
                    " "
                }
            );
        }
        print!("{}", "\n");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}
}
