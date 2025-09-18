extern crate plotters;
extern crate statrs;

use plotters::prelude::*;
use statrs::function::factorial::factorial;


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
    Ok(())
}
