#![feature(proc_macro_hygiene, decl_macro)]

use rand::Rng;

fn main() {
    run_server()
}

fn run_server() {
    rocket::ignite()
        .mount(
            "/calculate_diesel_usage_for_distance",
            rocket::routes![calculate_diesel_usage_for_distance],
        )
        .mount(
            "/probability_of_unit_injector_fail",
            rocket::routes![probability_of_unit_injector_fail],
        )
        .launch();
}

#[rocket::get("/?<vin>")]
fn probability_of_unit_injector_fail(vin: String) -> String {
    let _vin: String = vin;
    let random_number: u8 = rand::thread_rng().gen_range(0..101);
    let random_number_divided: f32 = random_number as f32 / 100.0;
    format!("{}", random_number_divided)
}

#[rocket::get("/?<distance>&<fuel_usage_per_100_km>&<year_of_production>")]
fn calculate_diesel_usage_for_distance(
    distance: usize,
    fuel_usage_per_100_km: usize,
    year_of_production: usize,
) -> String {
    let _year_of_production: usize = year_of_production;
    let fuel_consumption: f32 = distance as f32 / fuel_usage_per_100_km as f32;
    format!("{}", fuel_consumption)
}

#[cfg(test)]
mod tests {
    use crate::run_server;
    use rand::{rngs::ThreadRng, Rng};

    #[tokio::test]
    async fn test_probability_of_unit_injector_fail() -> Result<(), Box<dyn std::error::Error>> {
        std::thread::spawn(run_server);
        let content: String = reqwest::get(
            "http://localhost:8000/probability_of_unit_injector_fail?vin=wvwzzz3bz4e222206",
        )
        .await?
        .text()
        .await?;
        let parsed_content: f32 = content.parse::<f32>()?;
        let is_valid: bool = match parsed_content {
            value if ((0.0..=1.0).contains(&value) && value.is_sign_positive()) => true,
            _ => false,
        };
        assert!(is_valid);
        float_eq::assert_float_eq!(parsed_content, 0.0, rmax <= 1.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_calculate_diesel_usage_for_distance() -> Result<(), Box<dyn std::error::Error>> {
        std::thread::spawn(run_server);
        let mut rng: ThreadRng = rand::thread_rng();
        let random_distance: usize = rng.gen::<usize>();
        let random_fuel_usage: u8 = rng.gen_range(1..41);
        let content: String = reqwest::get(
            format!("http://localhost:8000/calculate_diesel_usage_for_distance?distance={}&fuel_usage_per_100_km={}&year_of_production=1920", random_distance, random_fuel_usage),
        )
        .await?
        .text()
        .await?;
        let parsed_content: f32 = content.parse::<f32>()?;
        let is_valid: bool = match parsed_content {
            value if (value >= 0.0 && value.is_sign_positive()) => true,
            _ => false,
        };
        assert!(is_valid);
        Ok(())
    }
}
