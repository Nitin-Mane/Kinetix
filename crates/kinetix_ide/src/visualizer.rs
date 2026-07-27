//! Live Matrix Inspector / Visualizer for MATLAB-style workspace state.

use kinetix_vm::{Value, KxMatrix};

#[derive(Debug, Clone)]
pub struct MatrixView {
    pub name: String,
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub enum InspectableValue {
    Matrix(MatrixView),
    Scalar(String),
    Vector(Vec<String>),
    Other(String),
}

pub fn inspect_value(name: &str, val: &Value) -> InspectableValue {
    match val {
        Value::Matrix(m) => {
            let m = m.lock().unwrap();
            let mut grid = vec![vec![0.0; m.cols]; m.rows];
            for r in 0..m.rows {
                for c in 0..m.cols {
                    grid[r][c] = m.get(r, c).unwrap_or(0.0);
                }
            }
            InspectableValue::Matrix(MatrixView {
                name: name.to_owned(),
                rows: m.rows,
                cols: m.cols,
                data: grid,
            })
        }
        Value::Int(i)   => InspectableValue::Scalar(i.to_string()),
        Value::Float(f) => InspectableValue::Scalar(f.to_string()),
        Value::Bool(b)  => InspectableValue::Scalar(b.to_string()),
        Value::String(s)=> InspectableValue::Scalar(s.to_string()),
        Value::Vec(v)   => {
            let v = v.lock().unwrap();
            let items = v.elements.iter().map(|e| e.to_string()).collect();
            InspectableValue::Vector(items)
        }
        other => InspectableValue::Other(other.to_string()),
    }
}
