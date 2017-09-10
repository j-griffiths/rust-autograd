extern crate ndarray;

use ndarray_ext::NdArray;
use tensor::Tensor;
use std::f32;
use ops;


pub struct LogSoftmax {
    pub axis: isize,
}

pub fn logsumexp(x: &NdArray, axis: usize) -> NdArray {
    let mut a = x.shape().to_vec();
    a[axis] = 1;
    let reduced_shape = a.as_slice();

    let max_fn = f32::max;
    let ref max = x.fold_axis(ndarray::Axis(axis), f32::MIN, move |&a, &b| max_fn(a, b))
        .into_shape(ndarray::IxDyn(reduced_shape))
        .unwrap();

    // subtract `max` to prevent overflow of exp
    let mut tmp = x - max;

    let exp = {
        tmp.mapv_inplace(|a| a.exp());
        tmp
    };

    // unwrap is safe
    let mut sum = exp.sum(ndarray::Axis(axis))
        .into_shape(ndarray::IxDyn(reduced_shape))
        .unwrap();

    let e = f32::consts::E;
    sum.mapv_inplace(move |a| a.log(e));
    sum += max;
    sum
}

pub fn log_softmax_forward(x: &NdArray, axis: usize) -> NdArray {
    x - &logsumexp(x, axis)
}

impl ops::Op for LogSoftmax {
    fn name(&self) -> &str {
        "LogSoftmax"
    }

    fn compute(&mut self, xs: &[&NdArray], train: bool) -> NdArray {
        let x = xs[0];
        let axis = if self.axis >= 0 {
            self.axis as usize
        } else {
            x.ndim() - 1
        };
        log_softmax_forward(x, axis)
    }

    fn lop(&self, gy: &Tensor, inputs: &[&Tensor], output: &Tensor) -> Vec<Option<Tensor>> {
        let sm = ops::exp(output);
        let sum = ops::reduce_sum(gy, 1, true);
        let mul = sm * sum;
        let gx = gy - mul;
        vec![Some(gx)]
    }
}