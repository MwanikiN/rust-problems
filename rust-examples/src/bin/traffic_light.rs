/*Traffic Light
Create an enum for red/yellow/green.
Return the appropriate action. */

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Caution",
        TrafficLight::Green => "Go",
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("Action for Red light: {}", action(light));

    let light = TrafficLight::Yellow;
    println!("Action for Yellow light: {}", action(light));

    let light = TrafficLight::Green;
    println!("Action for Green light: {}", action(light));
}