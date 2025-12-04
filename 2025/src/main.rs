mod day_1;
mod day_2;
mod day_3;
mod day_4;

use anyhow::Result;
use day_1::DayOne;
use day_2::DayTwo;
use day_3::DayThree;
use day_4::DayFour;
use std::path::{Path, PathBuf};

trait Day {
    fn new<P: AsRef<Path>>(p: P) -> Result<Self>
    where
        Self: Sized;
    fn part_one(&self) -> Result<()>;
    fn part_two(&self) -> Result<()>;
}

fn input_path(day: usize) -> Result<PathBuf> {
    let path = PathBuf::from(format!(
        "{}/src/day_{}/input.txt",
        env!("CARGO_MANIFEST_DIR"),
        day
    ));
    if !path.exists() {
        anyhow::bail!("Input path does not exist");
    }
    Ok(path)
}

fn main() -> Result<()> {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(DayOne::new(&input_path(1)?)?),
        Box::new(DayTwo::new(&input_path(2)?)?),
        Box::new(DayThree::new(&input_path(3)?)?),
        Box::new(DayFour::new(&input_path(4)?)?),
    ];

    if let Some(day) = std::env::args()
        .nth(1)
        .and_then(|day| day.parse::<usize>().map(|day_idx| &days[day_idx - 1]).ok())
    {
        day.part_one()?;
        day.part_two()?;
    } else {
        for day in days.iter() {
            day.part_one()?;
            day.part_two()?;
        }
    }

    Ok(())
}
