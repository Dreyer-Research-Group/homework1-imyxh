// SPDX-License-Identifier: MIT

mod quadgl_data;

fn _integrate_impl(n: usize, f: impl Fn(f64) -> f64) -> f64 {
    let mut ret: f64 = 0.0;
    let ws = quadgl_data::ws(n);
    let xs = quadgl_data::xs(n);
    for i in 0..n {
        ret += ws[i] * f(xs[i]);
    }
    return ret;
}

pub fn integrate(n: usize, f: impl Fn(f64) -> f64, x0: f64, x1: f64) -> f64 {
    let half = (x1 - x0) / 2.0;
    let mid = (x1 + x0) / 2.0;
    return half * _integrate_impl(n, |x| f(mid + half * x));
}
