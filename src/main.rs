struct Sedan;
impl LandCapable for Sedan {}
struct SUV;
impl LandCapable for SUV {}

trait LandCapable {
    fn drive(&self) {
        println!("Default drive")
    }
}

trait WaterCapable {
    fn float(&self);
}

// super trait
trait Amphibious: LandCapable + WaterCapable {}

struct Hovercraft;

impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {
    fn drive(&self) {
        println!("Hovercraft can drive")
    }
}
impl WaterCapable for Hovercraft {
    fn float(&self) {
        println!("Hovercraft can float");
    }
}

fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.drive();
    vehicle.float();
}

fn main() {
    let car = Sedan;
    road_trip(&car);
    let hover = Hovercraft;
    traverse_frozen_lake(&hover);
    
}