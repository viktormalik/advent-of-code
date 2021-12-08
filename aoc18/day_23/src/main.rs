extern crate petgraph;
extern crate regex;

use petgraph::Graph;
use regex::Regex;
use std::fs;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Bot {
    pos: (i32, i32, i32),
    range: i32,
}

fn intersect(pos1: i32, range1: i32, pos2: i32, range2: i32) -> bool {
    match pos1 < pos2 {
        true => pos1 + range1 >= pos2 - range2,
        false => pos2 + range2 >= pos1 - range1,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");
    let regex = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();

    let mut bots = Graph::<Bot, usize>::new();
    for line in input.trim().lines() {
        let cap: Vec<i32> = regex
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<i32>().unwrap())
            .collect();
        let new_bot_index = bots.add_node(Bot {
            pos: (cap[0], cap[1], cap[2]),
            range: cap[3],
        });

        for i in bots.node_indices() {
            if {
                let new_bot = bots.node_weight(new_bot_index).unwrap();
                let b = bots.node_weight(i).unwrap();
                intersect(new_bot.pos.0, new_bot.range, b.pos.0, b.range)
                    && intersect(new_bot.pos.1, new_bot.range, b.pos.1, b.range)
                    && intersect(new_bot.pos.2, new_bot.range, b.pos.2, b.range)
            } {
                bots.add_edge(new_bot_index, i, 1);
            }
        }
    }

    let strong = bots
        .raw_nodes()
        .iter()
        .max_by(|a, b| a.weight.range.cmp(&b.weight.range))
        .unwrap();
    let in_range = bots
        .raw_nodes()
        .iter()
        .filter(|b| {
            (strong.weight.pos.0 - b.weight.pos.0).abs()
                + (strong.weight.pos.1 - b.weight.pos.1).abs()
                + (strong.weight.pos.2 - b.weight.pos.2).abs()
                <= strong.weight.range
        })
        .count();
    println!("First: {}", in_range);

    let xrange_orig = (
        bots.raw_nodes()
            .iter()
            .map(|b| b.weight.pos.0 - b.weight.range)
            .min()
            .unwrap(),
        bots.raw_nodes()
            .iter()
            .map(|b| b.weight.pos.0 + b.weight.range)
            .max()
            .unwrap(),
    );
    let yrange_orig = (
        bots.raw_nodes()
            .iter()
            .map(|b| b.weight.pos.1 - b.weight.range)
            .min()
            .unwrap(),
        bots.raw_nodes()
            .iter()
            .map(|b| b.weight.pos.1 + b.weight.range)
            .max()
            .unwrap(),
    );
    let zrange_orig = (
        bots.raw_nodes()
            .iter()
            .map(|b| b.weight.pos.2 - b.weight.range)
            .min()
            .unwrap(),
        bots.raw_nodes()
            .iter()
            .map(|b| b.weight.pos.2 + b.weight.range)
            .max()
            .unwrap(),
    );

    let mut total_max = 0;
    let mut k = 0;
    let mut result = 0;
    loop {
        let mut ratio = 100000000;
        let xrange = (xrange_orig.0 / ratio, xrange_orig.1 / ratio);
        let yrange = (yrange_orig.0 / ratio, yrange_orig.1 / ratio);
        let zrange = (zrange_orig.0 / ratio, zrange_orig.1 / ratio);
        let mut candidates = vec![(xrange, yrange, zrange)];

        loop {
            //println!("{}: {}", ratio, candidates.len());
            let mut next_candidates: Vec<((i32, i32), (i32, i32), (i32, i32))> = Vec::new();
            let mut max = 0;
            let mut max_pos: (i32, i32, i32) = (0, 0, 0);
            for (xrange, yrange, zrange) in candidates {
                let mut cover = vec![
                    vec![
                        vec![0; (zrange.1 - zrange.0 + 1) as usize];
                        (yrange.1 - yrange.0 + 1) as usize
                    ];
                    (xrange.1 - xrange.0 + 1) as usize
                ];
                // Compute coverage with given ratio
                for x in xrange.0..xrange.1 + 1 {
                    for y in yrange.0..yrange.1 + 1 {
                        for z in zrange.0..zrange.1 + 1 {
                            for i in bots.node_indices() {
                                let b = bots.node_weight(i).unwrap();
                                let r = b.range / ratio
                                    + match ratio {
                                        1 => 0,
                                        _ => match k {
                                            0 => 1,
                                            _ => 3,
                                        },
                                    };

                                if (b.pos.0 / ratio - x).abs()
                                    + (b.pos.1 / ratio - y).abs()
                                    + (b.pos.2 / ratio - z).abs()
                                    <= r
                                {
                                    cover[(x - xrange.0) as usize][(y - yrange.0) as usize]
                                        [(z - zrange.0) as usize] += 1;
                                }
                            }
                        }
                    }
                }
                // Find maximum coverage
                let mut local_max = 0;
                for x in 0..cover.len() {
                    for y in 0..cover[x].len() {
                        for z in 0..cover[x][y].len() {
                            if cover[x][y][z] > local_max {
                                local_max = cover[x][y][z];
                                if local_max >= max {
                                    let xnorm = x as i32 + xrange.0;
                                    let ynorm = y as i32 + yrange.0;
                                    let znorm = z as i32 + zrange.0;
                                    if local_max > max
                                        || (xnorm.abs() + ynorm.abs() + znorm.abs())
                                            < (max_pos.0.abs() + max_pos.1.abs() + max_pos.2.abs())
                                    {
                                        max_pos = (xnorm, ynorm, znorm);
                                    }
                                    max = local_max;
                                }
                            }
                        }
                    }
                }
                // Choose candidates for the decreased ratio
                for x in 0..cover.len() {
                    for y in 0..cover[x].len() {
                        for z in 0..cover[x][y].len() {
                            if local_max > total_max
                                && cover[x][y][z] > total_max
                                && local_max - cover[x][y][z] == 0
                            {
                                let xnorm = x as i32 + xrange.0;
                                let ynorm = y as i32 + yrange.0;
                                let znorm = z as i32 + zrange.0;
                                next_candidates.push((
                                    (xnorm * 10, xnorm * 10 + 9),
                                    (ynorm * 10, ynorm * 10 + 9),
                                    (znorm * 10, znorm * 10 + 9),
                                ));
                            }
                        }
                    }
                }
            }
            if ratio == 1 {
                if max != 0 {
                    result = max_pos.0.abs() + max_pos.1.abs() + max_pos.2.abs();
                    //println!("{}", max);
                }
                total_max = max;
                break;
            }
            candidates = next_candidates;
            ratio /= 10;
        }
        if total_max == 0 {
            println!("Second: {}", result);
            break;
        }
        k += 1;
    }
}
