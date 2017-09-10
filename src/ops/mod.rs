extern crate ndarray;

use std::cell::RefCell;
use std::rc::Rc;
use tensor::{Tensor, RawTensor};
use ndarray_ext::NdArray;

pub mod dummy_op;
pub mod random_ops;
mod clip;
mod add_n;
mod log_softmax;
mod identity;
mod cmp_ops;
mod math_ops;
mod concat;
mod tile;
mod binary_ops;
mod mean_squared_error;
mod softmax;
mod sigmoid;
mod elu;
mod relu;
mod slice;
mod sigmoid_cross_entropy;
mod softmax_cross_entropy;
mod gather;
mod sparse_softmax_cross_entropy;
mod matmul;
mod swap_axes;
mod reshape;
mod reduction_ops;
mod squeeze;
mod expand_dims;


/// Represents a operation node in a computation graph.
/// `Tensor` wraps trait-object of this.
pub trait Op {
    // Name of this op
    fn name(&self) -> &str;

    // Computes "symbolic" Vector-Jacobian product, i.e. grads of
    // input nodes using output grad.
    // (see http://deeplearning.net/software/theano/tutorial/gradients.html)
    fn lop(&self, gy: &Tensor, inputs: &[&Tensor], output: &Tensor) -> Vec<Option<Tensor>>;

    // Actually runs this op.
    // num of inputs : N
    // num of outputs: 1
    fn compute(&mut self, xs: &[&NdArray], train: bool) -> NdArray;
}


#[inline]
fn apply_op<T: Op + 'static>(op: T, inputs: &[&Tensor]) -> Tensor {
    Tensor(Rc::new(RefCell::new(RawTensor {
        op: Box::new(op),
        inputs: inputs.iter().map(|a| (*a).clone()).collect::<Vec<Tensor>>(),
        param: None,
        rank: inputs
            .iter()
            .map(|a| a.borrow().rank)
            .max()
            .map(|a| a + 1)
            .unwrap_or(0),
    })))
}


// ---------------------------------------
// -- Ops to manipulate `Tensor` object --
// ---------------------------------------

#[inline]
pub fn asinh(x: &Tensor) -> Tensor {
    apply_op(math_ops::Asinh, &[x])
}


#[inline]
pub fn acosh(x: &Tensor) -> Tensor {
    apply_op(math_ops::Acosh, &[x])
}


#[inline]
pub fn atanh(x: &Tensor) -> Tensor {
    apply_op(math_ops::Atanh, &[x])
}


#[inline]
pub fn sinh(x: &Tensor) -> Tensor {
    apply_op(math_ops::Sinh, &[x])
}


#[inline]
pub fn cosh(x: &Tensor) -> Tensor {
    apply_op(math_ops::Cosh, &[x])
}


#[inline]
pub fn tanh(x: &Tensor) -> Tensor {
    apply_op(math_ops::Tanh, &[x])
}


#[inline]
pub fn asin(x: &Tensor) -> Tensor {
    apply_op(math_ops::Asin, &[x])
}


#[inline]
pub fn acos(x: &Tensor) -> Tensor {
    apply_op(math_ops::Acos, &[x])
}


#[inline]
pub fn atan(x: &Tensor) -> Tensor {
    apply_op(math_ops::Atan, &[x])
}


#[inline]
pub fn sin(x: &Tensor) -> Tensor {
    apply_op(math_ops::Sin, &[x])
}


#[inline]
pub fn cos(x: &Tensor) -> Tensor {
    apply_op(math_ops::Cos, &[x])
}


#[inline]
pub fn tan(x: &Tensor) -> Tensor {
    apply_op(math_ops::Tan, &[x])
}


#[inline]
pub fn add_n(xs: &[&Tensor]) -> Tensor {
    apply_op(add_n::AddN, xs)
}


#[inline]
pub fn identity(a: &Tensor) -> Tensor {
    apply_op(identity::Identity, &[a])
}

#[inline]
pub fn add(a: &Tensor, b: &Tensor) -> Tensor {
    apply_op(binary_ops::ElementwiseAdd, &[a, b])
}


#[inline]
pub fn sub(a: &Tensor, b: &Tensor) -> Tensor {
    apply_op(binary_ops::ElementwiseSub, &[a, b])
}


#[inline]
pub fn mul(a: &Tensor, b: &Tensor) -> Tensor {
    apply_op(binary_ops::ElementwiseMul, &[a, b])
}


#[inline]
pub fn div(a: &Tensor, b: &Tensor) -> Tensor {
    apply_op(binary_ops::ElementwiseDiv, &[a, b])
}


#[inline]
pub fn sqrt(x: &Tensor) -> Tensor {
    apply_op(math_ops::Sqrt, &[x])
}

#[inline]
pub fn pow(x: &Tensor, a: f32) -> Tensor {
    apply_op(math_ops::Pow { a: a }, &[x])
}


#[inline]
pub fn log(x: &Tensor, a: f32) -> Tensor {
    apply_op(math_ops::Log { a: a }, &[x])
}


#[inline]
pub fn exp(x: &Tensor) -> Tensor {
    apply_op(math_ops::Exp, &[x])
}


#[inline]
/// Returns binary tensor.
///
/// # Panics
/// When a.shape != b.shape.
pub fn equals(a: &Tensor, b: &Tensor) -> Tensor {
    apply_op(cmp_ops::Equals, &[a, b])
}


#[inline]
/// Takes argmax along specified axis.
pub fn argmax(x: &Tensor, axis: isize, keep_dim: bool) -> Tensor {
    let op = reduction_ops::ArgMax {
        axis: axis,
        keep_dim: keep_dim,
    };
    apply_op(op, &[x])
}


#[inline]
/// Expands dims.
pub fn expand_dims(x: &Tensor, axes: &[isize]) -> Tensor {
    let mut axes = axes.to_vec();
    axes.sort();
    apply_op(expand_dims::ExpandDims { axes: axes }, &[x])
}


#[inline]
/// Squeezes designated dim.
pub fn squeeze(x: &Tensor, axes: &[isize]) -> Tensor {
    let mut axes = axes.to_vec();
    axes.sort();
    apply_op(squeeze::Squeeze { axes: axes }, &[x])
}


#[inline]
/// Tiles input tensor along specified axis.
pub fn tile(x: &Tensor, axis: isize, num: usize) -> Tensor {
    let op = tile::Tile { axis: axis, num: num };
    apply_op(op, &[x])
}


#[inline]
/// Limits all elements so as to be within `[min, max]`
pub fn clip(x: &Tensor, min: f32, max: f32) -> Tensor {
    let op = clip::Clip { min: min, max: max };
    apply_op(op, &[x])
}


#[inline]
/// Take max along specified axis.
pub fn reduce_max(x: &Tensor, axis: isize, keep_dim: bool) -> Tensor {
    let op = reduction_ops::ReduceMax {
        axis: axis,
        keep_dim: keep_dim,
    };
    apply_op(op, &[x])
}


#[inline]
/// Take min along specified axis.
pub fn reduce_min(x: &Tensor, axis: isize, keep_dim: bool) -> Tensor {
    let op = reduction_ops::ReduceMin {
        axis: axis,
        keep_dim: keep_dim,
    };
    apply_op(op, &[x])
}


#[inline]
/// Take mean along specified axis.
pub fn reduce_mean(x: &Tensor, axis: isize, keep_dim: bool) -> Tensor {
    let op = reduction_ops::ReduceMean {
        axis: axis,
        keep_dim: keep_dim,
    };
    apply_op(op, &[x])
}


#[inline]
/// Take sum along specified axis.
pub fn reduce_sum(x: &Tensor, axis: isize, keep_dim: bool) -> Tensor {
    let op = reduction_ops::ReduceSum {
        axis: axis,
        keep_dim: keep_dim,
    };
    apply_op(op, &[x])
}

#[inline]
/// Returns gradient tensors for each variable.
///
/// # Arguments
/// * `objective` - Target of differentiation.
/// * `variables` - Variable tensors with which differentiate `objective`
///
/// # Returns
/// Symbolic gradient tensors corresponding to `variables` in the same order as `variables`
pub fn gradients(
    objective: &Tensor,
    variables: &[&Tensor],
    initial_grad: Option<&Tensor>,
) -> Vec<Tensor> {
    ::topology::symbolic_gradients(objective, variables, initial_grad)
}


#[inline]
/// Reshapes input tensor.
pub fn reshape(x: &Tensor, shape: &[isize]) -> Tensor {
    let mut minus_one_found = false;
    let shape = shape.iter().map(|&len| {
        if len == -1 {
            if minus_one_found {
                panic!("`shape` has two or more `-1` dim.");
            }
            minus_one_found = true;
            None
        } else if len < -1 {
            panic!("`shape` contains invalid dim size: {}", len);
        } else {
            Some(len as usize)
        }
    }).collect::<Vec<_>>();
    let op = reshape::Reshape {
        target_shape: shape
    };
    apply_op(op, &[x])
}


#[inline]
/// Returns 1-ranked tensor (vector)
pub fn flatten(x: &Tensor) -> Tensor {
    let op = reshape::Reshape {
        target_shape: vec![None]
    };
    apply_op(op, &[x])
}


#[inline]
/// Returns binary tensor.
pub fn greater(x: &Tensor, a: f32) -> Tensor {
    apply_op(cmp_ops::Greater { a: a }, &[x])
}


#[inline]
/// Returns binary tensor.
pub fn greater_equal(x: &Tensor, a: f32) -> Tensor {
    apply_op(cmp_ops::GreaterEqual { a: a }, &[x])
}


#[inline]
/// Returns binary tensor.
pub fn lesser(x: &Tensor, a: f32) -> Tensor {
    apply_op(cmp_ops::Lesser { a: a }, &[x])
}


#[inline]
/// Returns binary tensor.
pub fn lesser_equal(x: &Tensor, a: f32) -> Tensor {
    apply_op(cmp_ops::LesserEqual { a: a }, &[x])
}


#[inline]
/// Swaps two axes.
///
/// Swap axis `a` and axis `b` of `x`.
pub fn swap_axes(x: &Tensor, a: isize, b: isize) -> Tensor {
    apply_op(swap_axes::SwapAxes { a: a, b: b }, &[x])
}


#[inline]
/// Elementwise logistic sigmoid function.
pub fn sigmoid(x: &Tensor) -> Tensor {
    apply_op(sigmoid::Sigmoid, &[x])
}


#[inline]
/// Elementwise exponential linear unit function.
/// (https://arxiv.org/abs/1511.07289)
pub fn elu(x: &Tensor, alpha: f32) -> Tensor {
    apply_op(elu::ELU {alpha: alpha}, &[x])
}


#[inline]
/// Elementwise rectified linear unit function.
pub fn relu(x: &Tensor) -> Tensor {
    apply_op(relu::ReLU, &[x])
}


#[inline]
/// Log softmax function.
///
/// Take log.softmax along `axis`.
pub fn log_softmax(x: &Tensor, axis: isize) -> Tensor {
    let op = log_softmax::LogSoftmax { axis: axis };
    apply_op(op, &[x])
}


#[inline]
/// Softmax function.
///
/// Take softmax along `axis`.
pub fn softmax(x: &Tensor, axis: isize) -> Tensor {
    let op = softmax::Softmax { axis: axis };
    apply_op(op, &[x])
}


#[inline]
/// Just computes 0.5*(a-b)^2.
///
/// The performance is better than directly computing 0.5*(a-b)^2
/// if the gradient computation is required.
///
/// # Panics
/// When a.shape != b.shape.
pub fn mean_squared_error(a: &Tensor, b: &Tensor) -> Tensor {
    apply_op(mean_squared_error::MeanSquaredError, &[a, b])
}


#[inline]
/// Computes `binary_cross_entropy(sigmoid(y))`.
///
/// This function is better than that combination in that it can prevent
/// underflow of `log(sigmoid)`.
///
/// # Arguments
/// * `y` - Tensor with arbitrary shape
/// * `t` - Tensor with arbitrary shape
///
/// # Returns
/// Loss tensor with shape (batch_size, 1)
pub fn sigmoid_cross_entropy(y: &Tensor, t: &Tensor) -> Tensor {
    let op = sigmoid_cross_entropy::SigmoidCrossEntropy;
    apply_op(op, &[y, t])
}


#[inline]
/// Computes `categorical_cross_entropy(softmax(y))`.
///
/// This function is better than that combination in that it can prevent
/// underflow of `log(softmax)`.
///
/// # Arguments
/// * `y` - Tensor with shape (batch_size, num_classes)
/// * `t` - Tensor with shape (batch_size, num_classes)
///
/// # Returns
/// Loss tensor with shape (batch_size, 1)
pub fn softmax_cross_entropy(y: &Tensor, t: &Tensor) -> Tensor {
    let op = softmax_cross_entropy::SoftmaxCrossEntropy;
    apply_op(op, &[y, t])
}


#[inline]
/// A variant of `softmax_cross_entropy`.
///
/// The behavior of this function is same as `softmax_cross_entropy`
/// except that `t` is **not** batch of one-hot distributions but batch of ground truth label ids.
///
/// # Arguments
/// * `y` - Tensor with shape (batch_size, num_classes)
/// * `t` - Tensor with shape (batch_size, 1)
///
/// # Returns
/// Loss tensor with shape (batch_size, 1)
pub fn sparse_softmax_cross_entropy(y: &Tensor, t: &Tensor) -> Tensor {
    let op = sparse_softmax_cross_entropy::SparseSoftmaxCrossEntropy;
    apply_op(op, &[y, t])
}


#[inline]
/// Matrix multiplication.
///
/// `a` and `b` must be 2-ranked tensors.
pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
    apply_op(matmul::MatMul, &[a, b])
}


#[inline]
/// Slice op.
///
/// # Arguments
/// * `x` - Tensor with arbitrary shape.
/// * `starts` - Start indices for each dimensions
/// * `ends` - End indices for each dimensions. `-1` representing the last index is acceptable.
pub fn slice(x: &Tensor, starts: &[isize], ends: &[isize]) -> Tensor {
    assert_eq!(starts.len(), ends.len());
    let starts_ends = starts.iter().zip(ends.iter());

    let indices = starts_ends
        .map(|(s, e)| {
            ndarray::Si(*s, if *e == -1 { None } else { Some(*e) }, 1)
        })
        .collect::<Vec<ndarray::Si>>();

    let op = slice::Slice { indices: indices.into_boxed_slice() };

    apply_op(op, &[x])
}


#[inline]
/// Concat input tensors.
pub fn concat(tensors: &[&Tensor], axis: usize) -> Tensor {
    apply_op(concat::Concat { axis: axis }, tensors)
}


#[inline]
/// Gather slices.
///
/// For example, this can be used for vector lookup.
/// See https://www.tensorflow.org/api_docs/python/tf/gather.
///
/// # Arguments
/// * `param` - Target of slicing.
/// * `indices` - Index tensor with arbitrary shape.
/// * `axis` - Slices sub tensors along this axis.
///
/// # Returns
/// Tensor with shape `(param.shape[..axis] + indices.shape + param.shape[axis+1..])`
pub fn gather(param: &Tensor, indices: &Tensor, axis: isize) -> Tensor {
    let op = gather::Gather {
        axis: axis,
    };
    apply_op(op, &[indices, param])
}


#[inline]
/// Applies recurrent net unit to the input.
///
/// This func processes a time step in the batch of sequences in parallel.
///
/// # Arguments
/// * `x` - Input tensor for this step
/// * `rnn` - RNN struct
/// * `with_new_state` - If true, calls `rnn.reset_state()` before running a step
///
/// # Returns
/// Output of `rnn.step()`
pub fn rnn_step<T>(x: &Tensor, rnn: &mut T, with_new_state: bool) -> Tensor
where
    T: ::nn_impl::rnn::RNN,
{
    if with_new_state {
        rnn.reset_state();
    }
    rnn.step(x)
}