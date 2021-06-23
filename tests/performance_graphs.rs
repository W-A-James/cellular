use plotters::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::ops::Range;
const BUILD_GIF_PROFILING_FILE_NAME: &str = "build_gif_profiling.csv";
const RULE_STEP_PROFILING_FILE_NAME: &str = "rule_step_profiling.csv";

fn read_from_csv(file_name: &str) -> Result<HashMap<String, Vec<f64>>, std::io::Error> {
    let mut file = fs::File::open(file_name)?;
    let mut contents = String::new();
    let mut headers: Vec<&str> = Vec::new();
    let mut data: HashMap<String, Vec<f64>> = HashMap::new();
    file.read_to_string(&mut contents)?;
    let mut on_first_line = true;
    for line in contents.split('\n') {
        if on_first_line {
            on_first_line = false;
            for col_name in line.split(',') {
                let key = &(col_name[1..col_name.len() - 1]);
                data.insert(key.to_owned(), Vec::new());
                headers.push(key);
            }
        } else {
            let mut col_num = 0;
            for data_point in line.split(',') {
                let data_vec = match data.get_mut(headers[col_num]) {
                    Some(val) => val,
                    None => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Invalid key",
                        ))
                    }
                };
                match data_point.parse::<f64>() {
                    Ok(val) => data_vec.push(val),
                    Err(_) => continue,
                }
                col_num += 1;
            }
        }
    }

    Ok(data)
}

fn draw_graph(x_val: &Vec<f64>, y_val: &Vec<f64>, out_file_name: &str, x_range: Range<f64>, y_range: Range<f64>) {
    let mut points: Vec<(f64, f64)> = Vec::with_capacity(y_val.len());
    for i in 0..y_val.len() {
        points.push((x_val[i], y_val[i]));
    }

    let root = BitMapBackend::new(out_file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();
    chart
        .configure_mesh()
        .y_desc("Time")
        .y_label_formatter(&|x| format!("{:e}", x))
        .draw()
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            points.iter().map(|point| (point.0, point.1)),
            &BLACK,
        ))
        .unwrap();

    root.present().unwrap();
}

#[test]
fn build_rule_step_profiling_graph() {
    let out_file_name: &str = "rule_step_profiling_graph.png";
    let table = read_from_csv(RULE_STEP_PROFILING_FILE_NAME).unwrap();
    let widths = table.get("Width").unwrap();
    let times = table.get("Time(s)").unwrap();

    draw_graph(&widths, &times, out_file_name, 0f64..32768f64, 0f64..0.45f64);
}

#[test]
fn build_gif_profiling_graph() {
    let table = read_from_csv(BUILD_GIF_PROFILING_FILE_NAME);
}
