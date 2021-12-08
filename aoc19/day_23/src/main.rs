use intcode::*;
use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut programs: Vec<IntcodeProgram> = vec![];
    for i in 0..50 {
        programs.push(IntcodeProgram::load(&input));
        programs[i].run(&vec![i as i64]);
    }

    let mut queues: Vec<Vec<(i64, i64)>> = vec![vec![]; 50];
    let mut nat_packet = (0, 0);
    let mut last_nat = (0, 0);

    loop {
        let mut idle_count = 0;
        for i in 0..50 {
            let prog = &mut programs[i];
            if queues[i].len() > 0 {
                for packet in queues[i].drain(..) {
                    prog.run(&vec![packet.0, packet.1]);
                }
            } else {
                prog.run(&vec![-1]);
                idle_count += 1;
            }

            for packet in prog.stdout.drain(..).tuples::<(_, _, _)>() {
                if packet.0 == 255 {
                    if nat_packet == (0, 0) {
                        println!("First: {}", packet.2);
                    }
                    nat_packet = (packet.1, packet.2);
                } else {
                    queues[packet.0 as usize].push((packet.1, packet.2));
                }
            }
        }

        if idle_count == 50 {
            queues[0].push(nat_packet);
            if last_nat.1 == nat_packet.1 {
                println!("Second: {}", nat_packet.1);
                return;
            }
            last_nat = nat_packet;
        }
    }
}
