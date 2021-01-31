pub enum TrafficLight {
    Green,
    Red,
    Yellow
}

impl TrafficLight {
    pub fn time(&self) -> u8 {
        match self {
            TrafficLight::Green => 10,
            TrafficLight::Red => 20,
            TrafficLight::Yellow => 30
        }
    }
}
#[cfg(test)]
mod tests;

