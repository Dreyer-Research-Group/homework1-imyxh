// vim: set foldmethod=marker :
#import "homework.typ": *
#show: homework.with(
    title: [PHY604: homework 1],
    date: datetime.today(),
    name: "Ivy Huang",
    email: "ivy.m.huang@stonybrook.edu",
)

// problem 1 {{{
#problem(number: [1])[
    _understanding round-off error (no program required)_

    Consider a quadratic equation of the form $a x^2 + b x + c = 0$. The two
    solutions of this are:

    $
    x = (-b ± sqrt(b^2 - 4 a c)) / (2 a)  med.
    $

#subproblem(number: [1(a)])[
    Explain how this expression may be problematic with respect to roundoff
    errors if $b$ is much larger than $a$ and $c$. Recall that such errors
    often occur when subtracting close large numbers.
]]

If $b^2 >> 4 a c$, then $sqrt(b^2 - 4 a c) ≈ b$, in which case the $+$ solution
with end up with something asymptotic to $-b + b$ in the numerator, which is
prone to roundoff error. #miniqed

#problem(number: [1(b)])[
    Provide an alternative expression that will have smaller errors in the
    situation you describe in (a).
]

The $-$ solution is not a risk, so we ignore it for now. For the $+$ solution,
we multiply by one:

$
x = (-b + sqrt(b^2 - 4 a c)) / (2 a)
    ((b + sqrt(b^2 - 4 a c)) / (b + sqrt(b^2 - 4 a c)))
= ans((4 a c) / (2 a (b + sqrt(b^2 - 4 a c)))) med.
$

// }}}

// problem 2 {{{
#problem(number: [2])[
    _round-off error and accurate calculation of the exponential series_

    Consider the series expansion for an exponential function:

    $
    e^x ≈ S_n (x) := 1 + x/1! + x^2/2! + … + x^n/n! med.
    $

#subproblem(number: [2(a)])[
    Write a program that computes the exponential function using this series
    expansion for a given number of terms $n$.
]]

#ans[Done]. See the `exponential_series` function in `hw01.rs`.

#problem(number: [2(b)])[
    For $n$ ranging between 0 and 100, compare the result with the exponent
    calculated with a built-in function or function from a numerical library
    (e.g. `numpy.exp`) in the following way. Plot the error defined by

    $
    ve_n := abs(e^x - S_n (x)) / e^x
    $

    on a log-log plot for a large positive and large negative exponent
    (e.g., $x = 20$ and $x = -20$). Describe what you see.
]

The plot:

#figure(image("2b.svg", width: 100mm))

You can see that we actually never get to the correct answer for $x=-20$,
because somewhere along the process of going to $n = 100$, `factorial` on a
`u64` overflowed.

#problem(number: [2(c)])[
    Consider the following (trivial) equality: $e^(-x) = (e^(-1))^x$. Write a
    program that utilizes this equality to get a more accurate series
    expansion for large negative exponents. Plot $ve_n$ on a log-log plot to
    demonstrate that you have achieved this.
]

This one's easy; we just rerun with

```rust
fn exponential_series_alt(n: u64, x: f64) -> f64 {
    return 1.0 / exponential_series(n, -x);
}
```

#figure(image("2c.svg", width: 100mm))

// }}}

// problem 3 {{{
#problem(number: [3])[
    _errors in numerical differentiation_

    Calculate the derivative of the function $f(x) = sin x$ at the point $x =
    π\/4$ using the first-order forward difference. Plot on a log-log plot the
    error with respect to the analytical derivative for a wide range of
    $difference(x)$. Describe the behavior you see (especially for very small
    $difference(x)$) and the reason for the trends. How does it change if you
    use a second-order central difference? How about a fourth-order central
    difference?
]

First order forward:

#figure(image("3a.svg", width: 100mm))

It's pretty much a straight line on a log-log plot. I think this makes sense,
because the fractional error is (the magnitude of)

$
(
    (sin(x + difference(x)) - sin x) / difference(x)
    - cos x
) / (cos x)
=
((sin x) (cos difference(x) - 1)) / (difference(x) cos x) - 1
≈
((sin x) (- difference(x)^2)) / (2 difference(x) cos x) - 1
$

which is linear.

Second order central:

#figure(image("3b.svg", width: 100mm))

Fourth order central:

#figure(image("3c.svg", width: 100mm))

// }}}

// problem 4 {{{
#problem(number: [4])[
    _comparing methods of integration_

    Consider the variable

    $
    I = integral_0^1 (sin sqrt(100 x))^2 dd(x)
    $

#subproblem(number: [4(a)])[
    Plot the integrand over the range of the integral.
]]

As instructed:

#figure(image("4a.svg", width: 100mm))

// TODO
/*

#problem(number: [4(b)])[
    Write a program that uses the _adaptive trapezoid rule_ to calculate the
    integral to an approximate accuracy of $ve = 10^(-6)$, using the following
    procedure. Start with the trapezoid rule using a single subinterval. Double
    the number of subintervals and recalculate the integral. Continue to double
    the number of subintervals until the error is less than $10^(-6)$. Recall
    that the error is given by $ve_i = 1/3 (I_i - I_(i-1)$ where the number of
    subintervals $N_i$ used to calculate $I_i$ is twice that used to calculate
    $I_(i-1$. To make your implementation more efficient, use the fact that

    $
    I_i = 1/2 I_(i-1) + h_i sum_k f(a + k h_i)
    $

    where $h_i$ is the width of the subinterval for the $i$th iteration, and $k$
    runs over _odd numbers_ from 1 to $N_i - 1$.
]

// TODO

#problem(number: [4(c)])[
    Write a separate program that uses _Romberg integration_ to solve the
    integral, also to an accuracy of $10^(-6)$ using the following procedure.
    First calculate the integral with the trapezoid rule for 1 subinterval (as
    you did in part (b)); we will refer to this as step $i = 1$, and the result
    as $I_1 = R_(1,1)$. Then calculate $I_2 = R_(2,1)$ using 2 subintervals.
    Using these two results, we can construct an improved estimate of the
    integral as: $R_(2,2) = R_(2,1) + 1/3 (R_(2,1) - R_(1,1))$. In general

    $
    R_(i,m+1) = R_(i,m) + 1 / (4^m - 1) (R_(i,m) - R_(i-1,m)) med.
    $

    Therefore, for each iteration $i$ (where we double the number of
    subintervals), we can obtain improved approximations up to $m = i - 1$ with
    very minor extra work. For each $i$ and $m$, we can calculate the error at
    previous steps as

    $
    ve_(i,m) = 1 / (4^m - 1) (R_(i,m) - R_(i-1,m)) med.
    $

    Use these two equations to iterate until the error in $R_(i,i)$ is less than
    $10^(-6)$. How significant is the improvement with respect to number of
    subintervals necessary compared to the approach of part (b)?
]

// TODO

#problem(number: [4(d)])[
    Use the Gauss--Legendre approach to calculate the integral. What order
    (i.e., how many points) do you need to obtain an accuracy below $10^(-6)$?
    You can find tabulated weights and points online.
]

// TODO

// }}}
*/

// problem 5 {{{
#problem(number: [5])[
    _integration to ∞_

    Consider the gamma function,

    $
    Γ(a) = integral_0^∞ x^(a-1) e^(-x) dd(x) med.
    $

    We want to evaluate this numerically, and we will focus on $a > 1$. Consider
    a variable transformation of the form:

    $
    z = x / (x + c) med.
    $

    This will map $0 ≤ x < ∞$ to $0 ≤ z ≤ 1$, allowing us to do this integral
    numerically in terms of $z$. For convenience, we express the integrand as
    $vp(x) = x^(a - 1) e^(-x)$.

#subproblem(number: [5(a)])[
    Plot $vp(x)$ for $a in {2,3,4}$.
]]

As instructed:

#figure(image("5a.svg", width: 100mm))

// TODO

#problem(number: [5(b)])[
    For what value of $x$ is the integrand $vp(x)$ maximum?
]

// TODO

#problem(number: [5(c)])[
    Choose the value $c$ in our transformation such that the peak of the
    integrand occurs at $z = 1\/2$. What value is $c$?

    This choice spreads the interesting regions of integrand over the domain $0
    ≤ z ≤ 1$, making our numerical integration more accurate.
]

// TODO

#problem(number: [5(d)])[
    Find $Γ(a)$ for a few different values of $a > 1$ using any numerical
    integration method you wish, integrating from $z = 0$ to $z = 1$. Keep the
    number of points in your quadrature to a reasonable amount ($N ≤ 50$).

    Don't forget to include the factors you pick up when changing $dd(x)$ to
    $dd(z)$.

    Note that roundoff error may come into play in the integrand. Recognizing
    that you can write $x^(a - 1) = e^((a - 1) ln x)$ can help minimize this.
]

// TODO

// }}}

