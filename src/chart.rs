use crate::utils::*;
use burn::prelude::*;
use plotters::prelude::*;

/// The default caption for the chart
const CAPTION: &str = "fast-umap";

/// The default path where the plot will be saved
const PATH: &str = "plot.png";

/// Configuration structure for the chart, including caption, path, width, and height
#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub caption: String,
    pub path: String,
    pub width: u32,
    pub height: u32,
}

impl ChartConfig {
    /// Builder pattern for configuring the chart
    pub fn builder() -> ChartConfigBuilder {
        ChartConfigBuilder {
            caption: Some(CAPTION.to_string()),
            path: Some(PATH.to_string()),
            width: Some(1000),
            height: Some(1000),
        }
    }
}

impl Default for ChartConfig {
    /// Default implementation for ChartConfig with preset values
    fn default() -> Self {
        ChartConfig {
            caption: CAPTION.to_string(),
            path: PATH.to_string(),
            width: 1000,
            height: 1000,
        }
    }
}

/// Builder pattern for `ChartConfig` struct to allow flexible configuration
pub struct ChartConfigBuilder {
    caption: Option<String>,
    path: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

impl Default for ChartConfigBuilder {
    fn default() -> Self {
        ChartConfigBuilder {
            caption: Some(CAPTION.into()),
            path: Some(PATH.into()),
            width: None,
            height: None,
        }
    }
}

impl ChartConfigBuilder {
    /// Set the caption for the chart
    pub fn caption(mut self, caption: &str) -> Self {
        self.caption = Some(caption.to_string());
        self
    }

    /// Set the path where the chart will be saved
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// Set the width of the chart
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of the chart
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Build and return the final `ChartConfig`
    pub fn build(self) -> ChartConfig {
        ChartConfig {
            caption: self.caption.unwrap_or_else(|| CAPTION.to_string()),
            path: self.path.unwrap_or_else(|| PATH.to_string()),
            width: self.width.unwrap_or(1000),
            height: self.height.unwrap_or(1000),
        }
    }
}

type Float = f64;

/// Plot the 2D chart using the given tensor data and optional chart configuration
///
/// # Arguments
/// * `data` - A 2D tensor of data points to plot
/// * `config` - Optional custom chart configuration
pub fn chart_tensor<B: Backend>(data: Tensor<B, 2>, config: Option<ChartConfig>) {
    let data: Vec<Vec<Float>> = convert_tensor_to_vector(data);
    chart_vector(data, config);
}

/// Plot the 2D chart using the provided data and configuration
///
/// # Arguments
/// * `data` - A 2D vector of data points to plot
/// * `config` - Optional custom chart configuration
pub fn chart_vector(data: Vec<Vec<Float>>, config: Option<ChartConfig>) {
    let config = config.unwrap_or(ChartConfig::default());

    // Create a drawing area with a size of 800x600 pixels
    let root = BitMapBackend::new(&config.path, (config.width, config.height)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Define the range for x and y axes (include negative values)
    let min_x = data
        .iter()
        .flat_map(|v| v.iter().step_by(2)) // x values are at even indices
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap() as Float;

    let max_x = data
        .iter()
        .flat_map(|v| v.iter().step_by(2)) // x values are at even indices
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap() as Float;

    let min_y = data
        .iter()
        .flat_map(|v| v.iter().skip(1).step_by(2)) // y values are at odd indices
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap() as Float;

    let max_y = data
        .iter()
        .flat_map(|v| v.iter().skip(1).step_by(2)) // y values are at odd indices
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap() as Float;

    // Create a chart builder with specific size and axis ranges
    let mut chart = ChartBuilder::on(&root)
        .caption(config.caption, ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .unwrap();

    // Draw the x and y axis
    chart
        .configure_mesh()
        .x_desc("X Axis")
        .y_desc("Y Axis")
        .x_labels(10)
        .y_labels(10)
        .draw()
        .unwrap();

    // Plot each vector in the Vec<Vec<F>> as a series of dots
    chart
        .draw_series(data.iter().map(|values| {
            Circle::new(
                (values[0], values[1]),
                3,
                ShapeStyle {
                    color: RED.to_rgba(),
                    filled: false,
                    stroke_width: 1,
                },
            )
        }))
        .unwrap()
        .label("UMAP")
        .legend(move |(x, y)| {
            Circle::new(
                (x, y),
                5,
                ShapeStyle {
                    color: RED.to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
            )
        });

    // Draw the legend
    chart.configure_mesh().draw().unwrap();

    // Save the chart to a file
    root.present().unwrap();
}

/// Plot the loss curve over epochs and save it to a file
///
/// # Arguments
/// * `losses` - A vector of loss values over multiple epochs
/// * `output_path` - Path where the plot will be saved
pub fn plot_loss(losses: Vec<f64>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Calculate the min and max loss values
    let min_loss = losses.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_loss = losses.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // Add padding to the min and max values for better visualization
    let padding = 0.1; // 10% padding, adjust as needed
    let min_loss_with_padding = min_loss - padding * min_loss.abs();
    let max_loss_with_padding = max_loss + padding * max_loss.abs();

    // Create a drawing area with a width of 800px and a height of 600px
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart builder with padded Y-axis range
    let mut chart = ChartBuilder::on(&root)
        .caption("Loss Over Epochs", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .build_cartesian_2d(
            0..losses.len() as u32,
            min_loss_with_padding..max_loss_with_padding,
        )?;

    // Draw the chart axes and grid
    chart
        .configure_mesh()
        .y_desc("Loss")
        .x_desc("Epochs")
        .draw()?;

    // Plot the losses as a line
    chart
        .draw_series(LineSeries::new(
            (0..losses.len()).map(|x| (x as u32, losses[x])),
            &BLUE,
        ))?
        .label("Loss")
        .legend(move |(x, y)| PathElement::new(vec![(x, y)], &RED));

    // Draw the legend
    chart.configure_series_labels().draw()?;

    // Format Y-axis labels to handle small floats
    chart.configure_mesh().y_labels(10).draw()?;

    Ok(())
}
