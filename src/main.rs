mod cheats;

use std::f32::consts::PI;
use std::time::Duration;

use hs_hackathon::prelude::*;

use cheats::angles::Vector;
use cheats::approaching::Hint;
use cheats::positioning::Position;
use cheats::TeamColors;

const CAR: Color = Color::Blue;
const TARGET: Color = Color::Green;

#[allow(unused)]
struct MapState {
    car: Position,
    target: Position,
}

#[allow(unused)]
impl MapState {
    pub async fn infer(drone: &mut Camera) -> eyre::Result<Self> {
        unimplemented!()
    }

    async fn car_orientation(
        current: Position,
        drone: &mut Camera,
        motor: &mut MotorSocket,
        wheels: &mut WheelOrientation,
    ) -> eyre::Result<Vector> {
        unimplemented!()
    }
}

async fn launch_drone() -> eyre::Result<()> {
    Ok(())
}

async fn get_led_position(drone: &Camera, colour: Color) -> eyre::Result<Position> {
    let piccy: Frame = drone.snapshot().await?;

    let config = LedDetectionConfig::default();

    let leds = hs_hackathon::vision::detect(&piccy.0, &config)?;

    let targets: Vec<Led> = leds.into_iter().filter(|led| led.color == colour).collect();
    let our_target = targets.get(0).ok_or(eyre::Report::msg("No target found"))?;
    
    // Convert bbox to Position
    let pos: Position = our_target.bbox.into();
    Ok(pos)
}


async fn get_target_position(drone: &Camera) -> eyre::Result<Position> {
    let pos = get_led_position(drone, TARGET).await?;
    
    tracing::debug!("Found target at {:?}", pos);
    
    Ok(pos)
}

async fn get_car_position(drone: &Camera) -> eyre::Result<Position> {
    let pos = get_led_position(drone, CAR).await?;
    
    tracing::debug!("Found car at {:?}", pos);
    
    Ok(pos)
}


fn calculate_steering_angle(error_deg: f32) -> Angle {
    tracing::debug!("Angle deg: {:?}", error_deg);
    if error_deg > -0.01 {
        Angle::left()
    } else if error_deg < 0.01 {
        Angle::right()
    } else {
        Angle::straight()
    }

    /*
    let error_nom = error_rad / (360.0);

    error_nom.try_into().map_err(|e| tracing::debug!("Can't convert {:?} into angle", error_rad)).unwrap_or(Angle::straight())
    */
}

const THRESHOLD: u32 = 5;

#[hs_hackathon::main]
async fn main() -> eyre::Result<()> {
    let mut wheels = WheelOrientation::new().await?;
    let mut motor = MotorSocket::open().await?;
    let mut drone = Camera::connect().await?;

    // Launch drone
    // Adjust drone until both lights in picture

    launch_drone().await?;

    // Set steering angle to 0

    let mut last_car_pos: Position = Position{ x: 0, y: 0};

    loop {
        // Get car and target positions
        let target = get_target_position(&drone).await?;
        let new_car_pos = get_car_position(&drone).await?;

        // Check how close we are
        let distance_to_target = target.distance(&new_car_pos);
        if distance_to_target < THRESHOLD {
            tracing::debug!("Car is {} from target. Less than {}. Terminating.", distance_to_target, THRESHOLD);
            break;
        }

        let car_vector: Vector = (last_car_pos, new_car_pos).into();

        let target_vector: Vector = (new_car_pos, target).into();

        last_car_pos = new_car_pos;

        // Calculate angle
        let error_angle = car_vector.angle(target_vector) - 180.0;

        tracing::debug!("Calculated angle between car and target as {:?}", error_angle);

        // Calculate steering angle
        let steering_angle = calculate_steering_angle(error_angle as f32);

        // Set steering angle
        wheels.set(steering_angle).await?;

        // Move car forward 
        motor.move_for(Velocity::forward(), Duration::from_millis(500)).await?;
    }


    Ok(())
}




    /* 
    let mut wheels = WheelOrientation::new().await?;
    let mut motor = MotorSocket::open().await?;
    let mut drone = Camera::connect().await?;

    let mut machine = State::Turning;

    loop {
        machine.execute(&mut drone, &mut motor, &mut wheels).await?;
        tracing::debug!("{:?}", machine);
    }
        */


        /*
        #[derive(Debug)]
#[allow(unused)]
enum State {
    /// Turn the cars direction by doing consecutive front and back movements
    /// until the angle between the cars orientation and the target converges to be under
    /// a specified threshold
    Turning,
    /// Approach the car by doing incremental actions of approaching and measuring interleaved.
    /// So we approach the target a bit, measure if we decreased the distance, if yes repeat, if no
    /// then calibrate. We do this until we hit the target.
    Approaching,
    /// Simply idling on the target and identifying when the target moves away from our current
    /// position.
    Idle,
}

impl State {
    async fn execute(
        &mut self,
        drone: &mut Camera,
        motor: &mut MotorSocket,
        wheels: &mut WheelOrientation,
    ) -> eyre::Result<()> {
        match self {
            State::Turning => loop {
                unimplemented!()
            },
            State::Approaching => {
                let hint = cheats::approaching::auto(
                    &TeamColors {
                        car: CAR,
                        target: TARGET,
                    },
                    drone,
                    motor,
                    wheels,
                )
                .await?;

                *self = match hint {
                    Hint::TargetWasHit => Self::Idle,
                    Hint::OrientationIsOff => Self::Turning,
                };
            }
            State::Idle => {
                cheats::idling::auto(
                    &TeamColors {
                        car: CAR,
                        target: TARGET,
                    },
                    drone,
                    motor,
                    wheels,
                )
                .await?;

                *self = Self::Turning;
            }
        }

        Ok(())
    }
}
         */