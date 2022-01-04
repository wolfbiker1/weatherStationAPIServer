use super::trend::trend_handler::calc_trend;
use super::update::update_path_handler::load_current_measurements;
pub mod forecast_handler {
    use super::load_current_measurements;
    use ::inet::protocoll::http::HttpResponse;
    use atomic_float::AtomicF64;
    use std::sync::atomic::Ordering;
    use std::{thread, time};
    const TEN_MINUTES: std::time::Duration = time::Duration::from_secs(600);

    pub static OUTDOOR_TEMP_TREND: AtomicF64 = AtomicF64::new(0.0);
    pub static INDOOR_TEMP_TREND: AtomicF64 = AtomicF64::new(0.0);
    pub static PRESSURE_TREND: AtomicF64 = AtomicF64::new(0.0);
    pub static HUMIDITY_TREND: AtomicF64 = AtomicF64::new(0.0);
    pub static BRIGHTNESS_TREND: AtomicF64 = AtomicF64::new(0.0);

    fn update_static_values(field: &str, value: f64) {
        match field {
            "outdoor_temp" => OUTDOOR_TEMP_TREND.store(value, Ordering::SeqCst),
            "indoor_temp" => INDOOR_TEMP_TREND.store(value, Ordering::SeqCst),
            "pressure" => PRESSURE_TREND.store(value, Ordering::SeqCst),
            "humidity" => HUMIDITY_TREND.store(value, Ordering::SeqCst),
            "brightness" => BRIGHTNESS_TREND.store(value, Ordering::SeqCst),
            _ => {
                println!("sry no match.. {}", field);
            }
        }
    }
    pub fn load_trend_values(field: &str) -> f64 {
        match field {
            "outdoor_temp" => OUTDOOR_TEMP_TREND.load(Ordering::SeqCst),
            "indoor_temp" => INDOOR_TEMP_TREND.load(Ordering::SeqCst),
            "pressure" => PRESSURE_TREND.load(Ordering::SeqCst),
            "humidity" => HUMIDITY_TREND.load(Ordering::SeqCst),
            "brightness" => BRIGHTNESS_TREND.load(Ordering::SeqCst),
            _ => -1.0,
        }
    }

    pub fn main_worker() -> ! {
        const FIELDS: &[&str; 5] = &[
            "indoor_temp",
            "outdoor_temp",
            "pressure",
            "humidity",
            "brightness",
        ];
        println!("Forecast handler is ready...");
        loop {
            for field in FIELDS.iter() {}
            thread::sleep(TEN_MINUTES);
        }
    }

    pub fn calc_forecast() -> HttpResponse {
        let current_pressure = load_current_measurements("pressure");
        let current_temp = load_current_measurements("temp");
        let pressure_trend = load_trend_values("pressure");
        let p_0: f64 = current_pressure
            * f64::powf(
                1.0 - (0.0065 * 400.0) / (current_temp + 0.0065 * 400.0 + 273.15),
                -5.257,
            );
        let mut z: f64 = 0.0;
        if pressure_trend < 0.0 {
            z = (127.0 - (current_pressure * 0.12)) * 1.5;
        } else if pressure_trend > 0.0 {
            z = (185.0 - (current_pressure * 0.16)) * 1.5;
        } else {
            z = (144.0 - (current_pressure * 0.13)) * 1.5;
        }
        // let p = get sensor value...

        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", vec![z]),
        }
    }
}
