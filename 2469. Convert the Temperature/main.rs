impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        // with Rust expression & Vector 
        vec![
            celsius + 273.15,        // Kelvin
            celsius * 1.8 + 32.0,    // Fahrenheit
        ]
    }
}
