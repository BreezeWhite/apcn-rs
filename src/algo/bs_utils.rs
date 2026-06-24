// Binary Splitting Operation related utility functions

use crate::backend::{BigFloat, BigInt};
use rayon::prelude::*;
use std::thread;

pub enum BSMergeType {
    DEFAULT,
    LOG,
}

pub trait BinarySplit {
    fn compute_bs_base(a: u64) -> (BigInt, BigInt, BigInt);
    fn bs_finalize(binary_prec: u32, p: BigInt, q: BigInt, t: BigInt) -> BigFloat;
    fn merge_type() -> BSMergeType;
}

pub trait BinarySplitGeneric {
    type Value: Send + Sync + Clone;
    fn compute_base(&self, a: u64) -> Self::Value;
    fn merge(&self, left: Self::Value, right: Self::Value) -> Self::Value;
}

pub struct ClassicBinarySplitAdapter<S> {
    _marker: std::marker::PhantomData<S>,
}

unsafe impl<S> Send for ClassicBinarySplitAdapter<S> {}
unsafe impl<S> Sync for ClassicBinarySplitAdapter<S> {}

impl<S: BinarySplit> ClassicBinarySplitAdapter<S> {
    pub fn new() -> Self {
        ClassicBinarySplitAdapter {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S: BinarySplit> BinarySplitGeneric for ClassicBinarySplitAdapter<S> {
    type Value = (BigInt, BigInt, BigInt);

    fn compute_base(&self, a: u64) -> Self::Value {
        S::compute_bs_base(a)
    }

    fn merge(&self, left: Self::Value, right: Self::Value) -> Self::Value {
        let (pl, ql, tl) = left;
        let (pr, qr, tr) = right;
        match S::merge_type() {
            BSMergeType::DEFAULT => merge_default(pr, qr, tr, pl, ql, tl),
            BSMergeType::LOG => merge_log(pr, qr, tr, pl, ql, tl),
        }
    }
}

fn merge_default(
    mut pr: BigInt,
    qr: BigInt,
    tr: BigInt,
    mut pl: BigInt,
    mut ql: BigInt,
    mut tl: BigInt,
) -> (BigInt, BigInt, BigInt) {
    ql *= &qr;
    pr *= &pl;
    tl *= &qr;
    pl *= &tr;
    tl += pl;

    (pr, ql, tl)
}

fn merge_log(
    mut pr: BigInt,
    qr: BigInt,
    tr: BigInt,
    mut pl: BigInt,
    mut ql: BigInt,
    mut tl: BigInt,
) -> (BigInt, BigInt, BigInt) {
    ql *= &qr;
    tl *= &qr;
    tl *= &pr;
    pr *= &pl;
    pl *= &tr;
    tl += pl;
    (pr, ql, tl)
}

pub fn sub_binary_splitting_generic<T: BinarySplitGeneric>(
    a: u64,
    b: u64,
    context: &T,
) -> T::Value {
    if b - a == 1 {
        return context.compute_base(a);
    }

    let mid = (a + b) / 2;
    let left = sub_binary_splitting_generic(a, mid, context);
    let right = sub_binary_splitting_generic(mid, b, context);

    context.merge(left, right)
}

pub fn binary_splitting_generic_parallel<T: BinarySplitGeneric + Sync>(
    context: &T,
    terms: u64,
) -> T::Value {
    let thread_cnt = thread::available_parallelism().map(|n| n.get()).unwrap() / 2;
    let thread_cnt = thread_cnt.max(1);

    if terms < 500 || thread_cnt == 1 {
        return sub_binary_splitting_generic(0, terms, context);
    }

    let step_size = terms / thread_cnt as u64;

    let results: Vec<T::Value> = (0..thread_cnt)
        .into_par_iter()
        .map(|i| {
            let a = i as u64 * step_size;
            let b = if i == thread_cnt - 1 {
                terms
            } else {
                a + step_size
            };
            sub_binary_splitting_generic(a, b, context)
        })
        .collect();

    results
        .into_par_iter()
        .reduce_with(|left, right| context.merge(left, right))
        .unwrap()
}

pub fn binary_splitting<T: BinarySplit>(binary_prec: u32, terms: u64) -> BigFloat {
    let adapter = ClassicBinarySplitAdapter::<T>::new();
    let (p, q, t) = sub_binary_splitting_generic(0, terms, &adapter);
    T::bs_finalize(binary_prec, p, q, t)
}

pub fn binary_splitting_parallel<T: BinarySplit>(binary_prec: u32, terms: u64) -> BigFloat {
    let adapter = ClassicBinarySplitAdapter::<T>::new();
    let (p, q, t) = binary_splitting_generic_parallel(&adapter, terms);
    T::bs_finalize(binary_prec, p, q, t)
}
