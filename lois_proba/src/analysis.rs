use plotters::prelude::*;
use std::f64::consts::PI;
use statrs::distribution::*;

const SCATTER_SAMPLE: usize = 60000;
const HISTO_SAMPLE: usize = 100;

pub fn analyse(data: &Vec<f64>, yscale: f64, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    scatter_plot(data, name);
    qq_plot(data, name);
    histogram_plot(data, yscale, name);

    return Ok(());
}

pub fn scatter_plot(data: &Vec<f64>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut max: f64 = 0f64;
    let mut min: f64 = 0f64;
    for i in data {
        if *i > max {
            max = *i;
        }
        if *i < min {
            min = *i;
        }
    }

    let scatter_data: Vec<(f64, f64)> = data[0..SCATTER_SAMPLE]
        .iter()
        .enumerate()
        .map(|(x)| return ((x.0 as f64) / SCATTER_SAMPLE as f64, *x.1))
        .collect();

    let path = format!("plots/scatter_{}.png", name);
    let title = format!("Scatter {}, {} points", name, SCATTER_SAMPLE);

    let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(30)
        .caption(&title, ("sans-serif", 100.0))
        .build_cartesian_2d(0f64..1f64, min..max)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans-serif", 25.0))
        .draw()?;

    scatter_ctx.draw_series(
        scatter_data
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 1, BLUE.filled())),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    return Ok(());
}

//TODO add x = x line
pub fn qq_plot(data: &Vec<f64>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut max: f64 = 0f64;
    let mut min: f64 = 0f64;

    let sample_size = data.len() as usize;
    let mut qq_data = data.clone();

    let dev = std_deviation(&qq_data).unwrap();
    let mean = mean(&qq_data).unwrap();
    let mut normal = Normal::new(mean, dev).unwrap();
    
    for i in 0..sample_size {
        qq_data[i] =  normal.cdf(qq_data[i]) ;
    }

    for i in &qq_data {
        if *i > max {
            max = *i;
        }
        if *i < min {
            min = *i;
        }
    }

    qq_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let qq_data: Vec<(f64, f64)> = qq_data
        .iter()
        .enumerate()
        .map(|(x)| return (x.0 as f64 / sample_size as f64, *x.1))
        .collect();
        
    let path = format!("plots/qqplot_{}.png", name);
    let title = format!("QQ Plot {}", name);

    let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(30)
        .caption(&title, ("sans-serif", 100.0))
        .build_cartesian_2d(0f64..1f64, min..max)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans-serif", 25.0))
        .draw()?;

    scatter_ctx.draw_series(
        qq_data
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 3, BLUE.filled())),
    )?;

    scatter_ctx.draw_series(
        LineSeries::new(
        (0..2).map(|x| (x as f64, x as f64)),
        ShapeStyle { color: RGBAColor(255, 0, 0, 255f64), filled: true, stroke_width: 2 },
    ))?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    return Ok(());
}

pub fn histogram_plot(
    data: &Vec<f64>,
    yscale: f64,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut max: f64 = 0f64;
    let mut min: f64 = 0f64;
    for i in data {
        if *i > max {
            max = *i;
        }
        if *i < min {
            min = *i;
        }
    }

    let sample_size = data.len() as u32;

    let path = format!("plots/histogramme_{}.png", name);
    let title = format!("Histogramme {}", name);

    let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut histogram_ctx = ChartBuilder::on(&root)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(30)
        .caption(&title, ("sans-serif", 100.0))
        .build_cartesian_2d(
            (min..max)
                .step((max - min) / HISTO_SAMPLE as f64)
                .use_floor(),
            0f64..1f64,
        )?;
    histogram_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans-serif", 25.0))
        .draw()?;

    histogram_ctx.draw_series(
        Histogram::vertical(&histogram_ctx)
            .style(BLUE.stroke_width(3))
            .margin(0)
            .data(
                data.iter()
                    .map(|x| (*x, 1f64 / sample_size as f64 * HISTO_SAMPLE as f64 * yscale)),
            ),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    return Ok(());
}



fn mean(data: &Vec<f64>) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &Vec<f64>) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}
/*
Khi2
Moyenne
corrélation pearson

QQ plot DONE
histogramme DONE
scatter plot DONE





*/
