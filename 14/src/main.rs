use std::collections::HashMap;
use std::io::{BufRead, Read};
use std::str::*;

#[derive(Debug, Clone, PartialEq)]
struct Reagent {
    quant: i64,
    chem: String,
}
impl FromStr for Reagent {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let maybe_quant = iter.next();
        let maybe_chem = iter.next();
        maybe_quant
            .and_then(|qstr| qstr.parse::<i64>().ok())
            .and_then(|q| {
                maybe_chem.map(|c| Reagent {
                    quant: q,
                    chem: c.to_string(),
                })
            })
            .ok_or("Parsing Reagent failed".to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reaction {
    input: Vec<Reagent>,
    output: Reagent,
}

impl FromStr for Reaction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("=>");
        let maybe_input_str = iter.next();
        let maybe_output_str = iter.next();

        let maybe_input = maybe_input_str.and_then(|is| {
            is.split(',')
                .map(|r| Reagent::from_str(r))
                .collect::<Result<Vec<_>, _>>()
                .ok()
        });

        let maybe_output = maybe_output_str.and_then(|os| Reagent::from_str(os).ok());

        maybe_input
            .and_then(|i| {
                maybe_output.map(|o| Reaction {
                    input: i,
                    output: o,
                })
            })
            .ok_or("Parsing Reaction Failed".to_string())
    }
}

fn reaction_map(s: &String) -> HashMap<String, Reaction> {
    s.lines()
        .map(|l| Reaction::from_str(l).unwrap())
        .map(|r| (r.output.chem.to_string(), r))
        .collect()
}

fn traverse(
    input: &String,
    reac_map: &HashMap<String, Reaction>,
    current_needs: &mut Vec<Reagent>,
) {
    let mut ore_count: i64 = 0;

    while current_needs.len() > 0 && current_needs.iter().any(|x| x.quant > 0) {
        current_needs.sort_by(|a, b| {
            let a_remainder = {
                let is = reac_map.get(&a.chem).unwrap();
                a.quant % is.output.quant
            };
            let b_remainder = {
                let is = reac_map.get(&b.chem).unwrap();
                b.quant % is.output.quant
            };
            a_remainder.partial_cmp(&b_remainder).unwrap()
        });

        let mut new_reagents: Vec<Reagent> = Vec::new();
        {
            let r = current_needs.pop().unwrap();
            let mut is = reac_map.get(&r.chem).unwrap().clone();
            // this is ceiling integer division
            let scalar = (r.quant + is.output.quant - 1) / is.output.quant;
            for ir in is.input.iter_mut() {
                ir.quant *= scalar;
            }
            let extra = (is.output.quant * scalar - r.quant).max(0);
            new_reagents.append(&mut is.input);
            if extra > 0 {
                new_reagents.push(Reagent {
                    chem: r.chem,
                    quant: -extra,
                });
            }
        }

        current_needs.append(&mut new_reagents);
        current_needs.sort_by(|a, b| a.chem.partial_cmp(&b.chem).unwrap());
        current_needs.dedup_by(|ref mut a, ref mut b| {
            if a.chem == b.chem {
                b.quant += a.quant;
                true
            } else {
                false
            }
        });
        current_needs.retain(|x| {
            if x.chem == *input {
                ore_count += x.quant;
                false
            } else {
                true
            }
        });
    }
    current_needs.push(Reagent {
        quant: ore_count,
        chem: "ORE".to_string(),
    });
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

    let reac_map = reaction_map(&file_string);

    let mut needs: Vec<Reagent> = vec![Reagent {
        quant: 1,
        chem: "FUEL".to_string(),
    }];
    traverse(&"ORE".to_string(), &reac_map, &mut needs);
    println!("ORE: {}", needs.last().unwrap().quant);

    let tril: i64 = 1000000000000;
    let mut low: i64 = 0;
    let mut high: i64 = 10000000000000;
    while high > low {
        let mid = low + (high - low) / 2;
        let mut needs: Vec<Reagent> = vec![Reagent {
            quant: mid,
            chem: "FUEL".to_string(),
        }];
        traverse(&"ORE".to_string(), &reac_map, &mut needs);
        let ore = needs.last().unwrap().quant;
        println!("mid {} ore {}", mid, ore);
        if ore < tril {
            low = mid
        }
        else {
            high = mid;
        }
    }

}

#[cfg(test)]
mod tests {
    use reaction_map;
    use std::str::FromStr;
    use traverse;
    use Reaction;
    use Reagent;

    #[test]
    fn test1() {
        let str1 = "10 ore";
        let reg1 = Reagent::from_str(str1);
        let gt_reg1 = Reagent {
            quant: 10,
            chem: "ore".to_string(),
        };
        assert_eq!(reg1.unwrap(), gt_reg1);
        let str2 = "    100 gecs   ";
        let reg2 = Reagent::from_str(str2);
        let gt_reg2 = Reagent {
            quant: 100,
            chem: "gecs".to_string(),
        };
        assert_eq!(reg2.unwrap(), gt_reg2);
    }

    #[test]
    fn test2() {
        let test = "2 AB, 3 BC, 4 CA => 1 FUEL".to_string();
        let reac = Reaction::from_str(&test);
        let gt_reac = Reaction {
            input: vec![
                Reagent {
                    quant: 2,
                    chem: "AB".to_string(),
                },
                Reagent {
                    quant: 3,
                    chem: "BC".to_string(),
                },
                Reagent {
                    quant: 4,
                    chem: "CA".to_string(),
                },
            ],
            output: Reagent {
                quant: 1,
                chem: "FUEL".to_string(),
            },
        };
        assert_eq!(reac.unwrap(), gt_reac);
    }

    #[test]
    fn test3() {
        let test_str = r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

        let reac_map = reaction_map(&test_str.to_string());

        let mut needs: Vec<Reagent> = vec![Reagent {
            quant: 1,
            chem: "FUEL".to_string(),
        }];
        traverse(&"ORE".to_string(), &reac_map, &mut needs);
        //assert_eq!(needs.len(), 1);
        assert_eq!(needs.last().unwrap().quant, 165);
        assert_eq!(needs.last().unwrap().chem, "ORE");
    }
    #[test]
    fn test4() {
        let test_str = r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let reac_map = reaction_map(&test_str.to_string());

        let mut needs: Vec<Reagent> = vec![Reagent {
            quant: 1,
            chem: "FUEL".to_string(),
        }];
        traverse(&"ORE".to_string(), &reac_map, &mut needs);
        //assert_eq!(needs.len(), 1);
        assert_eq!(needs.last().unwrap().quant, 13312);
        assert_eq!(needs.last().unwrap().chem, "ORE");
    }
    #[test]
    fn test5() {
        let test_str = r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

        let reac_map = reaction_map(&test_str.to_string());

        let mut needs: Vec<Reagent> = vec![Reagent {
            quant: 1,
            chem: "FUEL".to_string(),
        }];
        traverse(&"ORE".to_string(), &reac_map, &mut needs);
        assert_eq!(needs.last().unwrap().quant, 180697);
        assert_eq!(needs.last().unwrap().chem, "ORE");
    }
}
