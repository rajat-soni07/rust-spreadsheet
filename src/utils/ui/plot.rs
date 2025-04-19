use plotters::prelude::*;

fn auto_range(data: &[(f64, f64)]) -> (std::ops::Range<f64>, std::ops::Range<f64>) {
    let (min_x, max_x) = data.iter().map(|(x, _)| *x).fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| (min.min(x), max.max(x)));
    let (min_y, max_y) = data.iter().map(|(_, y)| *y).fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| (min.min(y), max.max(y)));

    let x_range = if (max_x - min_x).abs() < std::f64::EPSILON {
        (min_x - 1.0)..(max_x + 1.0)
    } else {
        let margin = (max_x - min_x) * 0.1;
        (min_x - margin)..(max_x + margin)
    };

    let y_range = if (max_y - min_y).abs() < std::f64::EPSILON {
        (min_y - 1.0)..(max_y + 1.0)
    } else {
        let margin = (max_y - min_y) * 0.1;
        (min_y - margin)..(max_y + margin)
    };

    (x_range, y_range)
}


pub fn scatter_plot(data: &Vec<(f64, f64)>,path:&str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Extract bounds
    let (x_range, y_range) = auto_range(data);   

    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter Plot (Auto Axes)", ("Arial", 30).into_font())
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range, y_range)?;

    chart
        .configure_mesh()
        .x_desc("X Axis")
        .y_desc("Y Axis")
        .draw()?;

    chart
        .draw_series(data.iter().map(|(x, y)| Circle::new((*x, *y), 5, RED.filled())))?;

    Ok(())
}

pub fn line_plot(data: &Vec<(f64, f64)>,path:&str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;


    let (x_range, y_range) = auto_range(data);

    let mut chart = ChartBuilder::on(&root)
        .caption("Line Plot", ("Arial", 30).into_font())
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range, y_range)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(data.clone(), &BLUE))?;
    chart.draw_series(data.iter().map(|(x, y)| Circle::new((*x, *y), 3, BLUE.filled())))?;

    Ok(())
}