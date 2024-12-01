use std::iter::zip;

use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let example = aoc_2024::example(1)?;
    let (left, right) = into_number_vecs(example)?;
    let sum = sum_differences(left.iter(), right.iter());
    let similarity = count_same(left.iter(), right.iter());
    println!("example: {sum}, {similarity}");

    let input = aoc_2024::input(1)?;
    let (left, right) = into_number_vecs(input)?;
    let sum = sum_differences(left.iter(), right.iter());
    let similarity = count_same(left.iter(), right.iter());
    println!("input: {sum}, {similarity}");
    Ok(())
}

fn sum_differences<'a>(
    left: impl Iterator<Item = &'a u32>,
    right: impl Iterator<Item = &'a u32>,
) -> u32 {
    zip(left.sorted(), right.sorted())
        .map(|(l, r)| r.abs_diff(*l))
        .sum()
}

fn count_same<'a>(
    left: impl Iterator<Item = &'a u32>,
    right: impl Iterator<Item = &'a u32>,
) -> u32 {
    let counts = right.counts();
    left.map(|lnum| {
        let count: u32 = counts.get(lnum).map_or(0, |i| {
            TryInto::<u32>::try_into(*i).expect("number appears 2^32 times?!")
        });
        count * lnum
    })
    .sum()
}

fn into_number_vecs(example: String) -> anyhow::Result<(Vec<u32>, Vec<u32>)> {
    let (left, right) = example
        .split('\n')
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()?
                .into_iter()
                .collect_tuple::<(u32, u32)>()
                .ok_or(anyhow!("not exactly 2 elements"))
        })
        .collect::<Result<(Vec<u32>, Vec<u32>), _>>()?;

    Ok((left, right))
}
