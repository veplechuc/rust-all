#![allow(dead_code)]

#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Miles {
    value: u32,
}
#[derive(Debug)]
struct Mph {
    value: u32,
}

// impl Kmh {
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }
 
trait DistanceInXHours {
    type Distance;
    fn distance_in_three_hours(&self, x_hour: &u32) -> Self::Distance;
}

impl DistanceInXHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self, x_hour: &u32) -> Self::Distance{
        Self::Distance {
            value: self.value * x_hour,
        }
    }
}

impl DistanceInXHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self, x_hour: &u32) -> Self::Distance {
        Self::Distance {
            value: self.value * x_hour,
        }
    }
}
 

fn main() {
    let speed = Kmh { value: 90 };
    let x_hours = 5;
    let distance = speed.distance_in_three_hours(&x_hours); //this wont throws an error because i32 implements copy trait
    println!("At {:?}, you will travel {:?} in {:?} hours",  speed, distance, x_hours);

    let speed_mph = Mph { value: 90 };
    let distance_miles = speed_mph.distance_in_three_hours(&x_hours);
    println!("At {:?}, you will travel {:?} in {:?} hours", speed_mph, distance_miles, x_hours);
}