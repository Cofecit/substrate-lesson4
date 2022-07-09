#![allow(dead_code, unused_imports)]

// Traffic light enum, u32 indicate light time duration 
pub enum TrafficLight {
    Red(u32),
    Green(u32),
    Yellow(u32),
}

impl TrafficLight {
    // method to get light time duration
    pub fn time(&self) -> u32 {
        match *self {
            TrafficLight::Red(x) => x,
            TrafficLight::Green(x) => x,
            TrafficLight::Yellow(x) => x,
        }
    }
}

/////////////////////////// test case for traffic lights ///////////////////////////
#[cfg(test)]
mod tests {
    use crate::traffic_light::TrafficLight;
    
    #[test]
    fn test_traffic_lights() {

        // create read, green and yellow lights
        let red = TrafficLight::Red(30);
        let green = TrafficLight::Green(30);
        let yellow = TrafficLight::Yellow(3);

        // assert the light time
        assert_eq!(red.time(), 30);
        assert_eq!(green.time(), 30);
        assert_eq!(yellow.time(), 3);
    }
}