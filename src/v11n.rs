use plotly::{Bar, Layout, Plot};
use std::collections::HashMap;

pub fn visualize_data(data: Vec<HashMap<String, serde_json::Value>>) -> String {
    let x: Vec<String> = data
        .iter()
        .map(|row| {
            row.get("column_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        })
        .collect();

    let y: Vec<f64> = data
        .iter()
        .map(|row| row.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0))
        .collect();

    let trace = Bar::new(x, y);
    let layout = Layout::new().title("Transaction Data");
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    plot.to_html()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn v11n_works() {
        let data = vec![
            {
                let mut map = HashMap::new();
                map.insert("column_name".to_string(), serde_json::json!("A"));
                map.insert("value".to_string(), serde_json::json!(10.0));
                map
            },
            {
                let mut map = HashMap::new();
                map.insert("column_name".to_string(), serde_json::json!("B"));
                map.insert("value".to_string(), serde_json::json!(20.0));
                map
            },
        ];

        let html = visualize_data(data);
        println!("{}", html);
    }
}
