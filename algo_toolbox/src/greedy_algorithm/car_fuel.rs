// use math::round;

pub struct Car {
    distance: u32,
    max_miles: u32,
}

#[derive(Debug)]
pub struct Stations {
    number: u16,
    locations: Vec<u32>,
}

pub fn car_fuelling(car: Car, gas_stations: Stations) {
    let refill_times = ((car.distance / car.max_miles) as f32).ceil() as u32;

    if refill_times > 1 {
        let mut num_refill = 0;
        let mut distance_covered = car.max_miles;
        let mut gas_stations_used: Vec<u32> = Vec::new();

        for (index, station) in gas_stations.locations.iter().enumerate() {
            if station < &distance_covered && (index < gas_stations.locations.len() - 1) {
                if gas_stations.locations[index + 1] > distance_covered {
                    distance_covered += car.max_miles;
                    num_refill += 1;
                    gas_stations_used.push(*station);
                }
            } else if station < &distance_covered && (index + 1 == gas_stations.locations.len()) {
                distance_covered += car.max_miles;
                num_refill += 1;
                gas_stations_used.push(*station);
            }
        }

        if distance_covered >= car.distance {
            println!("total miles covered =====>>> {}", distance_covered);
            println!("number of refills ===>>> {}", num_refill);
            println!("refill stations used ===>>> {:?}", gas_stations_used);
            println!("<<<<<<<<<<<<<<<<<<<<||||||||||||||||||||||||>>>>>>>>>>>>>>>>>>>>>");
        } else {
            println!("out of fuel ==> -1");
        }
    } else {
        println!("gas statio not needed ===> 0")
    }
}

pub fn execute() {
    let car = Car {
        distance: 900,
        max_miles: 400,
    };
    let gas_station = Stations {
        number: 4,
        locations: vec![200, 375, 500, 750],
    };
    car_fuelling(car, gas_station);

    let car1 = Car {
        distance: 10,
        max_miles: 3,
    };
    let gas_station1 = Stations {
        number: 4,
        locations: vec![1, 2, 5, 9],
    };
    car_fuelling(car1, gas_station1);

    let car2 = Car {
        distance: 200,
        max_miles: 250,
    };
    let gas_station2 = Stations {
        number: 2,
        locations: vec![100, 150],
    };
    car_fuelling(car2, gas_station2);
}
