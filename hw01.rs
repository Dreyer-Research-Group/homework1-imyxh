extern crate plotters;
extern crate statrs;

use plotters::prelude::*;
use statrs::function::factorial::factorial;
use std::f64::consts::PI;


// for my diagnostics only
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
    return (
        - (x + 2.0 * dx).sin()
        + 8.0 * (x + dx).sin()
        - 8.0 * (x - dx).sin()
        + (x - 2.0 * dx).sin()
    ) / (12.0 * dx);
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
        .collect::<Vec<(f64, f64)>>()
    ;
    let eps_n20 = (0..100)
        .map(|n| {
            (
                n as f64,
                ((-20_f64).exp() - exponential_series(n, -20.0)).abs() / (-20_f64).exp(),
            )
        })
        .collect::<Vec<(f64, f64)>>()
    ;
    // plot them (yes, this is a ridiculous amount of boilerplate)
    // (in the future I'll have functions for all this but I'm rushed rn)
    let f = SVGBackend::new("2b.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("2(b)", ("Libertinus Serif", 20))
        .build_cartesian_2d((1f64..100f64).log_scale(), (0f64..1000f64).log_scale())?
    ;
    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(eps_p20.clone(), &BLUE))?
        .label("x=+20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE))
    ;
    chart
        .draw_series(LineSeries::new(eps_n20, &RED))?
        .label("x=-20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED))
    ;
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?
    ;
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
        .collect::<Vec<(f64, f64)>>()
    ;
    let f = SVGBackend::new("2c.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("2(c)", ("Libertinus Serif", 20))
        .build_cartesian_2d((1f64..100f64).log_scale(), (0f64..1000f64).log_scale())?
    ;
    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(eps_p20, &BLUE))?
        .label("x=+20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE))
    ;
    chart
        .draw_series(LineSeries::new(eps_n20_alt, &RED))?
        .label("x=-20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED))
    ;
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?
    ;
    f.present()?;

    // 3(a) (first order)
    // ------------------
    let fofd_err = (1..1000)
        .map(|n| {
            (
                (0.001_f64 * (n as f64)) as f64,
                (
                    fofd_sinx(0.001_f64 * (n as f64), PI / 4.0)
                    - (PI / 4.0).cos()
                ).abs() / (PI / 4.0).cos()
            )
        })
        .collect::<Vec<(f64, f64)>>()
    ;
    let f = SVGBackend::new("3a.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("fofd fractional error", ("Libertinus Serif", 20))
        .build_cartesian_2d((0.001f64..1f64).log_scale(), (0f64..0.5f64).log_scale())?
    ;
    chart
        .configure_mesh()
        .x_desc("Δx")
        .draw()?
    ;
    chart
        .draw_series(LineSeries::new(fofd_err, &BLUE))?
        .label("x=-20")
    ;
    f.present()?;

    // 3(b) (second order)
    // ------------------
    let socd_err = (1..1000)
        .map(|n| {
            (
                (0.001_f64 * (n as f64)) as f64,
                (
                    socd_sinx(0.001_f64 * (n as f64), PI / 4.0)
                    - (PI / 4.0).cos()
                ).abs() / (PI / 4.0).cos()
            )
        })
        .collect::<Vec<(f64, f64)>>()
    ;
    let f = SVGBackend::new("3b.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("socd fractional error", ("Libertinus Serif", 20))
        .build_cartesian_2d((0.001f64..1f64).log_scale(), (0f64..0.5f64).log_scale())?
    ;
    chart
        .configure_mesh()
        .x_desc("Δx")
        .draw()?
    ;
    chart
        .draw_series(LineSeries::new(socd_err, &BLUE))?
        .label("x=-20")
    ;
    f.present()?;

    // 3(c) (fourth order)
    // ------------------
    let ffffocd_err = (1..1000)
        .map(|n| {
            (
                (0.001_f64 * (n as f64)) as f64,
                (
                    ffffocd_sinx(0.001_f64 * (n as f64), PI / 4.0)
                    - (PI / 4.0).cos()
                ).abs() / (PI / 4.0).cos()
            )
        })
        .collect::<Vec<(f64, f64)>>()
    ;
    let f = SVGBackend::new("3c.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("ffffocd fractional error", ("Libertinus Serif", 20))
        .build_cartesian_2d((0.001f64..1f64).log_scale(), (0f64..0.5f64).log_scale())?
    ;
    chart
        .configure_mesh()
        .x_desc("Δx")
        .draw()?
    ;
    chart.draw_series(LineSeries::new(ffffocd_err, &BLUE))?;
    f.present()?;

    // 4(a)
    // ----
    let f = SVGBackend::new("4a.svg", (400, 300)).into_drawing_area();
    let _ = f.fill(&WHITE);
    let f = f.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&f)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("4(a)", ("Libertinus Serif", 20))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?
    ;
    chart
        .configure_mesh()
        .x_desc("x")
        .y_desc("sin(sqrt(100x))^2")
        .draw()?
    ;
    chart.draw_series(LineSeries::new(
        (0..10000).map(|x| {(
            0.0001 * (x as f64),
            (100.0 * 0.0001 * (x as f64)).sqrt().sin().powi(2)
        )}),
        &BLUE
    ))?;
    f.present()?;

    Ok(())
}
