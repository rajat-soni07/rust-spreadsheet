//! Data visualization utilities for the spreadsheet application.
//!
//! This module provides functions to create visual representations of spreadsheet data
//! using the plotters library. It supports different plot types including scatter plots
//! and line plots with automatic axis scaling.
use plotters::prelude::*;

/// Calculates appropriate axis ranges for a data series.
///
/// This function automatically determines suitable x and y axis ranges based on the
/// provided data points, adding margins around the data for better visualization.
///
/// # Arguments
/// * `data` - Slice of (x, y) coordinate pairs to analyze
///
/// # Returns
/// A tuple of (x_range, y_range) where each range is a `std::ops::Range<f64>`.
/// suitable for use with plotters
fn auto_range(data: &[(f64, f64)]) -> (std::ops::Range<f64>, std::ops::Range<f64>) {
    let (min_x, max_x) = data
        .iter()
        .map(|(x, _)| *x)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
            (min.min(x), max.max(x))
        });
    let (min_y, max_y) = data
        .iter()
        .map(|(_, y)| *y)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| {
            (min.min(y), max.max(y))
        });

    let x_range = if (max_x - min_x).abs() < f64::EPSILON {
        (min_x - 1.0)..(max_x + 1.0)
    } else {
        let margin = (max_x - min_x) * 0.1;
        (min_x - margin)..(max_x + margin)
    };

    let y_range = if (max_y - min_y).abs() < f64::EPSILON {
        (min_y - 1.0)..(max_y + 1.0)
    } else {
        let margin = (max_y - min_y) * 0.1;
        (min_y - margin)..(max_y + margin)
    };

    (x_range, y_range)
}

/// Creates a scatter plot from a set of data points and saves it to a file.
///
/// This function generates a scatter plot where each data point is rendered as
/// a separate circle. It automatically scales the axes to fit the data.
///
/// # Arguments
/// * `data` - Slice of (x, y) coordinate pairs to plot
/// * `path` - Path where the plot image will be saved
///
/// # Returns
/// `Ok(())` if the operation was successful, or an error otherwise
pub fn scatter_plot(data: &[(f64, f64)], path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    chart.draw_series(
        data.iter()
            .map(|(x, y)| Circle::new((*x, *y), 5, RED.filled())),
    )?;

    Ok(())
}

/// Creates a line plot from a set of data points and saves it to a file.
///
/// This function generates a line plot where data points are connected with lines
/// and each point is marked with a small circle. It automatically scales the axes
/// to fit the data.
///
/// # Arguments
/// * `data` - Slice of (x, y) coordinate pairs to plot
/// * `path` - Path where the plot image will be saved
///
/// # Returns
/// `Ok(())` if the operation was successful, or an error otherwise
pub fn line_plot(data: &[(f64, f64)], path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    chart.draw_series(LineSeries::new(data.to_owned(), &BLUE))?;
    chart.draw_series(
        data.iter()
            .map(|(x, y)| Circle::new((*x, *y), 3, BLUE.filled())),
    )?;

    Ok(())
}
