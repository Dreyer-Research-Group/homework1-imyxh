// SPDX-License-Identifier: MIT
extern crate plotters;
extern crate statrs;

use plotters::prelude::*;
use statrs::function::factorial::factorial;
use std::f64::consts::PI;

mod quadgl;

// for my diagnostics only
#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn exponential_series(n: u64, x: f64) -> f64 {
    let mut ret: f64 = 1.0;
    for j in 1..=n {
        ret += x.powi(j.try_into().unwrap()) / factorial(j);
    }
    return ret;
}

fn exponential_series_alt(n: u64, x: f64) -> f64 {
    return 1.0 / exponential_series(n, -x);
}

fn fofd_sinx(dx: f64, x: f64) -> f64 {
    return ((x + dx).sin() - x.sin()) / dx;
}

fn socd_sinx(dx: f64, x: f64) -> f64 {
    return ((x + dx).sin() - (x - dx).sin()) / (2.0 * dx);
}

fn ffffocd_sinx(dx: f64, x: f64) -> f64 {
    return (-(x + 2.0 * dx).sin() + 8.0 * (x + dx).sin() - 8.0 * (x - dx).sin()
        + (x - 2.0 * dx).sin())
        / (12.0 * dx);
}

fn trapezoid_integrate(eps: f64, f: fn(f64) -> f64, x0: f64, x1: f64) -> (u64, f64) {
    let mut subs: u64 = 1;
    let mut ret: f64 = (x1 - x0) * (f(x0) + f(x1)) / 2.0;
    let mut error: f64 = f64::INFINITY;
    while error.abs() >= eps {
        error = -ret / 3.0;
        subs *= 2;
        ret /= 2.0;
        let h: f64 = (x1 - x0) / (subs as f64);
        ret += h
            * (1..subs)
                .step_by(2)
                .map(|k| f(x0 + (k as f64) * h))
                .sum::<f64>();
        error += ret / 3.0;
        if subs > (u16::MAX as u64) {
            eprintln!("maximum iterations hit in trapezoid_integrate; error = {error}");
            break;
        }
    }
    return (subs, ret);
}

fn romberg_integrate(eps: f64, f: fn(f64) -> f64, x0: f64, x1: f64) -> (usize, f64) {
    // If we keep a 2D array for R_(i,m), it's O(n^2) in space; instead, we squash them into one
    // array `r` (O(n) in space) and keep a record `t` of the index of the last R_(i,1).
    let mut r: Vec<f64> = Vec::new();
    let mut t: usize = 0;
    let mut i: usize = 1;
    let mut subs: usize = 1;
    let mut err: f64 = f64::INFINITY;
    // R_(1,1)
    r.push((x1 - x0) * (f(x0) + f(x1)) / 2.0);
    while err.abs() >= eps {
        i += 1;
        subs *= 2;
        let h: f64 = (x1 - x0) / (subs as f64);
        // R_(i+1, 1)
        r.push(
            r[t] / 2.0
                + h * (1..subs)
                    .step_by(2)
                    .map(|k| f(x0 + (k as f64) * h))
                    .sum::<f64>(),
        );
        for m in 1..i {
            let pre = *(r.last().unwrap());
            err = (pre - r[t + m - 1]) / (((1_usize << (2 * m)) - 1) as f64);
            // R_(i, m+1)
            r.push(pre + err);
        }
        t = r.len() - i;
        if subs > (u16::MAX as usize) {
            eprintln!("maximum iterations hit in romberg_integrate; err = {err}");
            break;
        }
    }
    return (subs, r.pop().unwrap());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2(b)
    // ----
    // generate points
    let eps_p20 = (0..100)
        .map(|n| {
            (
                n as f64,
                (20_f64.exp() - exponential_series(n, 20.0)).abs() / 20_f64.exp(),
            )
        })
        .collect::<Vec<(f64, f64)>>();
    let eps_n20 = (0..100)
        .map(|n| {
            (
                n as f64,
                ((-20_f64).exp() - exponential_series(n, -20.0)).abs() / (-20_f64).exp(),
            )
        })
        .collect::<Vec<(f64, f64)>>();
    // plot them (yes, this is a ridiculous amount of boilerplate)
    // (in the future I'll have functions for all this but I'm rushed rn)
    let f = SVGBackend::new("2b.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("2(b)", ("Libertinus Serif", 20))
        .build_cartesian_2d((1f64..100f64).log_scale(), (0f64..1000f64).log_scale())?;
    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(eps_p20.clone(), &BLUE))?
        .label("x=+20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart
        .draw_series(LineSeries::new(eps_n20, &RED))?
        .label("x=-20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;
    f.present()?;

    // 2(c)
    // ----
    let eps_n20_alt = (0..100)
        .map(|n| {
            (
                n as f64,
                ((-20_f64).exp() - exponential_series_alt(n, -20.0)).abs() / (-20_f64).exp(),
            )
        })
        .collect::<Vec<(f64, f64)>>();
    let f = SVGBackend::new("2c.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("2(c)", ("Libertinus Serif", 20))
        .build_cartesian_2d((1f64..100f64).log_scale(), (0f64..1000f64).log_scale())?;
    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(eps_p20, &BLUE))?
        .label("x=+20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart
        .draw_series(LineSeries::new(eps_n20_alt, &RED))?
        .label("x=-20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;
    f.present()?;

    // 3(a) (first order)
    // ------------------
    let fofd_err = (1..1000)
        .map(|n| {
            (
                (0.001_f64 * (n as f64)) as f64,
                (fofd_sinx(0.001_f64 * (n as f64), PI / 4.0) - (PI / 4.0).cos()).abs()
                    / (PI / 4.0).cos(),
            )
        })
        .collect::<Vec<(f64, f64)>>();
    let f = SVGBackend::new("3a.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("fofd fractional error", ("Libertinus Serif", 20))
        .build_cartesian_2d((0.001f64..1f64).log_scale(), (0f64..0.5f64).log_scale())?;
    chart.configure_mesh().x_desc("Δx").draw()?;
    chart
        .draw_series(LineSeries::new(fofd_err, &BLUE))?
        .label("x=-20");
    f.present()?;

    // 3(b) (second order)
    // ------------------
    let socd_err = (1..1000)
        .map(|n| {
            (
                (0.001_f64 * (n as f64)) as f64,
                (socd_sinx(0.001_f64 * (n as f64), PI / 4.0) - (PI / 4.0).cos()).abs()
                    / (PI / 4.0).cos(),
            )
        })
        .collect::<Vec<(f64, f64)>>();
    let f = SVGBackend::new("3b.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("socd fractional error", ("Libertinus Serif", 20))
        .build_cartesian_2d((0.001f64..1f64).log_scale(), (0f64..0.5f64).log_scale())?;
    chart.configure_mesh().x_desc("Δx").draw()?;
    chart
        .draw_series(LineSeries::new(socd_err, &BLUE))?
        .label("x=-20");
    f.present()?;

    // 3(c) (fourth order)
    // ------------------
    let ffffocd_err = (1..1000)
        .map(|n| {
            (
                (0.001_f64 * (n as f64)) as f64,
                (ffffocd_sinx(0.001_f64 * (n as f64), PI / 4.0) - (PI / 4.0).cos()).abs()
                    / (PI / 4.0).cos(),
            )
        })
        .collect::<Vec<(f64, f64)>>();
    let f = SVGBackend::new("3c.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("ffffocd fractional error", ("Libertinus Serif", 20))
        .build_cartesian_2d((0.001f64..1f64).log_scale(), (0f64..0.5f64).log_scale())?;
    chart.configure_mesh().x_desc("Δx").draw()?;
    chart.draw_series(LineSeries::new(ffffocd_err, &BLUE))?;
    f.present()?;

    // 4(a)
    // ----
    fn inner4(x: f64) -> f64 {
        return (100.0 * x).sqrt().sin().powi(2);
    }
    let f = SVGBackend::new("4a.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("4(a)", ("Libertinus Serif", 20))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    chart
        .configure_mesh()
        .x_desc("x")
        .y_desc("sin(sqrt(100x))^2")
        .draw()?;
    chart.draw_series(LineSeries::new(
        (0..10000).map(|x| (0.0001 * (x as f64), inner4(0.0001 * (x as f64)))),
        &BLUE,
    ))?;
    f.present()?;

    // 4(b)
    // ----
    let (subs, ret) = trapezoid_integrate(1E-6, inner4, 0.0, 1.0);
    println!("4b: subintervals = {subs}, ret = {ret}");

    // 4(c)
    // ----
    let (subs, ret) = romberg_integrate(1E-6, inner4, 0.0, 1.0);
    println!("4c: subintervals = {subs}, ret = {ret}");

    // 4(d)
    // ----
    let mut ret: f64;
    for n in 2..=12 {
        ret = quadgl::integrate(n, inner4, 0.0, 1.0);
        println!("4d: n = {n}, ret = {ret}");
    }

    // 5(a)
    // ----
    fn phi(a: u64, x: f64) -> f64 {
        x.powi((a - 1).try_into().unwrap()) * (-x).exp()
    }
    let f = SVGBackend::new("5a.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("5(a)", ("Libertinus Serif", 20))
        .build_cartesian_2d(0f64..10f64, 0f64..1.4f64)?;
    chart.configure_mesh().x_desc("x").y_desc("phi").draw()?;
    chart
        .draw_series(LineSeries::new(
            (0..10000).map(|x| (0.001 * (x as f64), phi(2, 0.001 * (x as f64)))),
            &RED,
        ))?
        .label("a=2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(
            (0..10000).map(|x| (0.001 * (x as f64), phi(3, 0.001 * (x as f64)))),
            &GREEN,
        ))?
        .label("a=3")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    chart
        .draw_series(LineSeries::new(
            (0..10000).map(|x| (0.001 * (x as f64), phi(4, 0.001 * (x as f64)))),
            &BLUE,
        ))?
        .label("a=4")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;
    f.present()?;

    // 5(d)
    // ----
    fn phiz(c: u64, z: f64) -> f64 {
        (c as f64) * ((c as f64) * z).powi(c as i32)
            * (- (c as f64) * z / (1.0 - z)).exp()
            * (1.0 - z).powi(-2 - c as i32)
    }
    let mut ret: f64;
    for c in 1..=3 {
        ret = quadgl::integrate(50, |z| phiz(c, z), 0.0, 1.0);
        println!("5d: a = {c}+1, ret = {ret}");
    }

    Ok(())
}

