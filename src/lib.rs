extern crate rayon;
extern crate rug;

use rayon::prelude::*;
use rug::Integer;

fn align(mut x: usize) -> usize {
    x |= x >> 32;
    x |= x >> 16;
    x |= x >> 8;
    x |= x >> 4;
    x |= x >> 2;
    x |= x >> 1;
    x ^= x >> 1;
    x
}

fn product_tree(s: &[Integer]) -> Vec<Vec<Integer>> {
    let len = s.len();
    if len == 1 {
        vec![s.to_vec()]
    } else {
        let (l, r) = if len & (len - 1) == 0 {
            s.split_at(len >> 1)
        } else {
            s.split_at(align(len))
        };
        let (mut l, mut r) = rayon::join(|| product_tree(l), || product_tree(r));

        // align
        if l.len() > r.len() {
            let padding = r.last().unwrap().clone();
            (0..l.len() - r.len()).for_each(|_| r.push(padding.clone()));
        }

        l.par_iter_mut().zip(r.par_iter_mut()).for_each(|(l, r)| {
            l.append(r);
        });
        l.push(vec![l.last().unwrap().par_iter().product()]);
        l
    }
}

pub fn batch_gcd(s: &Vec<Integer>) -> Vec<Integer> {
    let mut prods = product_tree(&s);
    let result = prods.pop().unwrap();
    prods
        .iter()
        .rev()
        .fold(result, |acc, level| {
            level
                .par_iter()
                .enumerate()
                .map(|(i, x)| &acc[i >> 1] % Integer::from(x * x))
                .collect::<Vec<_>>()
        })
        .par_iter()
        .zip(s.par_iter())
        .map(|(rem, x)| Integer::from(x.gcd_ref(&Integer::from(rem / x))))
        .collect::<Vec<_>>()
}
