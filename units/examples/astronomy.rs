use unit::{AstronomicalUnits, Days, Kilometers, KilometersPerSecond, LightYears, Seconds};

fn main() {
    let earth_velocity: KilometersPerSecond = unit::Quantity::new(29.78);
    let time = Days::new(1.0);
    let time_sec: Seconds = time.to();
    let distance: Kilometers = earth_velocity * time_sec;

    assert!((distance.value() - 2_573_395.2).abs() < 5_000.0);

    let proxima = LightYears::new(4.24);
    let au: AstronomicalUnits = proxima.to();
    assert!(au.value() > 200_000.0);
}
