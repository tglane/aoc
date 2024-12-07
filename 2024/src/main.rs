mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

use anyhow::Result;

fn main() -> Result<()> {
    if let Some(day) = std::env::args()
        .nth(1)
        .map(|day| day.parse::<i32>().ok())
        .flatten()
    {
        match day {
            1 => day_1::run()?,
            2 => day_2::run()?,
            3 => day_3::run()?,
            4 => day_4::run()?,
            5 => day_5::run()?,
            6 => day_6::run()?,
            7 => day_7::run()?,
            _ => (),
        }
    }

    Ok(())
}
