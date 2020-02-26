use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, Read, Write};

fn pt(i: usize, size: usize) -> Vec<usize> {
    // okay now we need to gather our inputs
    let p_idx = i + 1;

    let subp_len = p_idx;
    let offset = 1;

    let pos_start = subp_len - offset;
    // let neg_start = pos_start + 2 * subp_len;

    let mut pos: Vec<usize> = Vec::new();
    let mut s = 0;
    while s < size {
        for i in pos_start..pos_start + subp_len {
            if i + s < size {
                pos.push(s + i);
            }
        }
        s += 4 * subp_len;
    }
    pos
}
fn nt(i: usize, size: usize) -> Vec<usize> {
    let p_idx = i + 1;

    let subp_len = p_idx;
    let offset = 1;

    let pos_start = subp_len - offset;
    let neg_start = pos_start + 2 * subp_len;

    let mut neg: Vec<usize> = Vec::new();
    let mut s = 0;
    while s < size {
        for i in neg_start..neg_start + subp_len {
            if i + s < size {
                neg.push(s + i);
            }
        }
        s += 4 * subp_len;
    }
    neg
}
fn get_val_rec(
    level: i32,
    idx: usize,
    input: &Vec<i32>,
    // p_table: &Vec<Vec<usize>>,
    // n_table: &Vec<Vec<usize>>,
    table: &mut HashMap<(i32, usize), i32>,
) -> i32 {
    if level == 0 {
        return input[idx];
    }
    if table.contains_key(&(level, idx)) {
        return table[&(level, idx)];
    }
    let mut pos_res: Vec<i32> = Vec::new();
    for &p in &pt(idx, input.len()) {
        let res = get_val_rec(level - 1, p, input, table);
        pos_res.push(res);
    }
    let mut neg_res: Vec<i32> = Vec::new();
    for &n in &nt(idx, input.len()) {
        let res = get_val_rec(level - 1, n, input, table);
        neg_res.push(res);
    }

    let res = i32::abs(pos_res.iter().sum::<i32>() - neg_res.iter().sum::<i32>()) % 10;
    table.insert((level, idx), res);
    res
}

fn get_val(
    level: i32,
    idx: usize,
    input: &Vec<i32>,
    // p_table: &Vec<Vec<usize>>,
    // n_table: &Vec<Vec<usize>>,
    table: &mut HashMap<(i32, usize), i32>,
) -> i32 {
    if level == 0 {
        return input[idx];
    }
    //  if table.contains_key(&(level, idx)) {
    //      return table[&(level, idx)];
    //  }

    //  let mut pos_res: Vec<i32> = Vec::new();
    //  for &p in &p_table[idx] {
    //      let res = get_val(level - 1, p, input, p_table, n_table, table);
    //      pos_res.push(res);
    //  }
    //  let mut neg_res: Vec<i32> = Vec::new();
    //  for &n in &n_table[idx] {
    //      let res = get_val(level - 1, n, input, p_table, n_table, table);
    //      neg_res.push(res);
    //  }

    let mut r: Vec<(i32, usize)> = Vec::new();
    r.push((level, idx));

    while !r.is_empty() {
        eprintln!("q: {}", r.len());
        let (l, i) = r.remove(0);
        if table.contains_key(&(l, i)) {
            continue;
        }

        if l == 0 {
            table.insert((0, i), input[i]);
            continue;
        }

        let n_deps = nt(i, input.len())
            .iter()
            .filter(|&ni| !table.contains_key(&(l - 1, *ni)))
            .map(|x| *x)
            .collect::<Vec<usize>>();
        let p_deps = pt(i, input.len())
            .iter()
            .filter(|&pi| !table.contains_key(&(l - 1, *pi)))
            .map(|x| *x)
            .collect::<Vec<usize>>();

        if !n_deps.is_empty() || !p_deps.is_empty() {
            for nd in n_deps {
                r.push((l - 1, nd));
            }
            for pd in p_deps {
                r.push((l - 1, pd));
            }
            r.push((l, i));
            r.sort();
            r.dedup();
            continue;
        }

        let mut pos_res: Vec<i32> = Vec::new();
        for &p in &pt(i, input.len()) {
            let res = table[&(l - 1, p)];
            pos_res.push(res);
        }
        let mut neg_res: Vec<i32> = Vec::new();
        for &n in &nt(i, input.len()) {
            let res = table[&(l - 1, n)];
            neg_res.push(res);
        }

        let res = i32::abs(pos_res.iter().sum::<i32>() - neg_res.iter().sum::<i32>()) % 10;
        table.insert((l, i), res);
        // if we have what we need, calculate
        // otherwise push dependencies and try again
    }

    let res = table[&(level, idx)];
    res
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Provide one argument with path to the program");
    }

    let file = std::fs::File::open(&args[1]).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut file_string = String::new();
    reader
        .read_to_string(&mut file_string)
        .expect("Unable to read file");

    let base_input: Vec<i32> = file_string
        .chars()
        .into_iter()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as i32)
        .collect();

    let message_offset = base_input.iter().take(7).fold(0, |acc, x| acc * 10 + x) as usize;
    eprintln!("mo: {}", message_offset);

    let mut input: Vec<i32> = base_input
        .iter()
        .cycle()
        .map(|x| *x)
        .take(base_input.len() * 10000 )
        .collect();

    // let base_pattern: Vec<i32> = vec![0, 1, 0, -1];

    // let mut consolidated_filter = vec![0; input.len()];
    // for i in 0..input.len() {
    //     let mut current_pattern: Vec<i32> = Vec::new();
    //     for p in &base_pattern {
    //         for _ in (1)..(i + 2) {
    //             current_pattern.push(*p);
    //         }
    //     }

    //     for (c, p) in consolidated_filter.iter_mut().zip(current_pattern.iter()) {
    //         *c += p;
    //     }
    // }
    // let new = input
    //     .iter()
    //     .zip(consolidated_filter.iter())
    //     .map(|(i, f)| i * f)
    //     .collect::<Vec<_>>();

    // let p_table = {
    //     let mut pt: Vec<Vec<usize>> = Vec::new();
    //     for i in 0..input.len() {
    //         eprintln!("pt: {}", i);
    //         // okay now we need to gather our inputs
    //         let p_idx = i + 1;

    //         let subp_len = p_idx;
    //         let offset = 1;

    //         let pos_start = subp_len - offset;
    //         // let neg_start = pos_start + 2 * subp_len;

    //         let mut pos: Vec<usize> = Vec::new();
    //         let mut s = 0;
    //         while s < input.len() {
    //             for i in pos_start..pos_start + subp_len {
    //                 if i + s < input.len() {
    //                     pos.push(s + i);
    //                 }
    //             }
    //             s += 4 * subp_len;
    //         }
    //         pt.push(pos);
    //     }
    //     pt
    // };
    // let n_table = {
    //     let mut nt: Vec<Vec<usize>> = Vec::new();
    //     for i in 0..input.len() {
    //         // okay now we need to gather our inputs
    //         let p_idx = i + 1;

    //         let subp_len = p_idx;
    //         let offset = 1;

    //         let pos_start = subp_len - offset;
    //         let neg_start = pos_start + 2 * subp_len;

    //         let mut neg: Vec<usize> = Vec::new();
    //         let mut s = 0;
    //         while s < input.len() {
    //             for i in neg_start..neg_start + subp_len {
    //                 if i + s < input.len() {
    //                     neg.push(s + i);
    //                 }
    //             }
    //             s += 4 * subp_len;
    //         }
    //         nt.push(neg);
    //     }
    //     nt
    // };

    let mut table: HashMap<(i32, usize), i32> = HashMap::new();
    //for i in message_offset..message_offset + 8 {
    for i in 0..8 {
        let res = get_val_rec(5, i, &input, /*&p_table, &n_table,*/ &mut table);
        eprintln!("{}", res);
    }

    // let mut lookups: Vec<(usize, usize)> = Vec::new();
    // for i in 0..input.len() {
    //     // let mut sub_pattern: Vec<i32> = Vec::new();
    //     // for p in &base_pattern {
    //     //     for _ in (1)..(i + 2) {
    //     //         sub_pattern.push(*p);
    //     //     }
    //     // }
    //     // let current_pattern = std::iter::once(0 as i32)
    //     //     .chain(sub_pattern.into_iter().cycle())
    //     //     .take(input.len())
    //     //     .collect::<Vec<_>>();
    //     // eprintln!(
    //     //     "{:?}",
    //     //     current_pattern
    //     //         .iter()
    //     //         .cycle()
    //     //         .skip(1)
    //     //         .take(8)
    //     //         .map(|x| *x)
    //     //         .collect::<Vec<i32>>()
    //     // );

    //     let p_idx = i + 1;

    //     let subp_len = p_idx;
    //     let offset = 1;

    //     let ones_start = subp_len - offset;
    //     let negs_start = ones_start + 2 * subp_len;

    //     let ones_slice = usize::min(ones_start, input.len());
    //     let negss_slice = usize::min(negs_start, input.len());
    //     lookups.push((ones_slice, negss_slice));
    // }

    // for iter in 0..100 {
    //     let mut new: Vec<i32> = Vec::new();
    //     for i in 0..input.len() {
    //         // let mut sub_pattern: Vec<i32> = Vec::new();
    //         // for p in &base_pattern {
    //         //     for _ in (1)..(i + 2) {
    //         //         sub_pattern.push(*p);
    //         //     }
    //         // }
    //         // let current_pattern = std::iter::once(0 as i32)
    //         //     .chain(sub_pattern.into_iter().cycle())
    //         //     .take(input.len())
    //         //     .collect::<Vec<_>>();
    //         // eprintln!(
    //         //     "{:?}",
    //         //     current_pattern
    //         //         .iter()
    //         //         .cycle()
    //         //         .skip(1)
    //         //         .take(8)
    //         //         .map(|x| *x)
    //         //         .collect::<Vec<i32>>()
    //         // );

    //         let p_idx = i + 1;

    //         let subp_len = p_idx;
    //         let (p, n) = lookups[i];

    //         let pos: i32 = input[p..]
    //             .chunks(subp_len)
    //             .step_by(4)
    //             .flatten()
    //             .sum::<i32>();

    //         let neg: i32 = input[n..]
    //             .chunks(subp_len)
    //             .step_by(4)
    //             .flatten()
    //             .sum::<i32>();

    //         new.push(i32::abs(pos - neg) % 10);
    //     }
    //     input = new;
    //     // eprintln!("{:?}", input);

    //     eprintln!("{}", iter);
    // }
    // let results = input
    //     .iter()
    //     .skip(message_offset)
    //     .take(8)
    //     .collect::<Vec<_>>();

    // eprintln!("{:?}", input);
    // eprintln!("{:?}", results);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}

    #[test]
    fn test2() {}
}
