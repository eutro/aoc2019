use crate::io;
use crate::io::stdin;
use itertools::Itertools;
use std::iter;

const OFFSET_DIGITS: usize = 7;
const NTH_OUTPUT: usize = 100;
const MESSAGE_LEN: usize = 8;
const REPEAT_INPUT: usize = 10_000;

fn fft_nth(signal: &Vec<u32>, phases: usize) -> Vec<u32> {
    let mut out_signal = signal.clone();
    for _ in 0..phases {
        out_signal = (0..out_signal.len())
            .map(|i| i + 1)
            .map(|pos| {
                out_signal
                    .iter()
                    .zip(
                        vec![0, 1, 0, -1]
                            .into_iter()
                            .flat_map(|i: i32| iter::repeat(i).take(pos))
                            .cycle()
                            .skip(1),
                    )
                    .map(|(&x, y)| x as i32 * y)
                    .sum::<i32>()
                    .abs() as u32
                    % 10
            })
            .collect_vec();
    }
    out_signal
}

fn fft_message(signal: &Vec<u32>, phases: usize, offset: usize, length: usize) -> Vec<u32> {
    let mut out_signal = signal[offset..].to_vec();
    out_signal.reverse();
    for _ in 0..phases {
        let mut last = 0_u32;
        out_signal = out_signal
            .into_iter()
            .map(|x| {
                last = (last + x) % 10;
                last
            })
            .collect_vec();
    }
    out_signal.reverse();
    out_signal.into_iter().take(length).collect_vec()
}

#[no_mangle]
pub fn day_16() {
    let stdin = stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let signal = buf
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect_vec();

    io::println!(
        "Output: {}",
        fft_nth(&signal, NTH_OUTPUT)
            .iter()
            .take(MESSAGE_LEN)
            .join("")
    );

    let message_offset = signal
        .iter()
        .take(OFFSET_DIGITS)
        .fold(0, |total, &d| total * 10 + d) as usize;

    let true_input = iter::repeat(signal)
        .take(REPEAT_INPUT)
        .flat_map(|v| v.into_iter())
        .collect_vec();

    io::println!(
        "Message: {}",
        fft_message(&true_input, NTH_OUTPUT, message_offset, MESSAGE_LEN)
            .iter()
            .join("")
    );
}
