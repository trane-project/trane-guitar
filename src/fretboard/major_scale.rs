use anyhow::Result;
use lazy_static::lazy_static;
use trane::course_builder::{music::scales::ScaleType, CourseBuilder};
use ustr::Ustr;

use crate::fretboard::fretboard_exploration::FretboardExplorationCourse;

lazy_static! {
    pub static ref COURSE_ID: Ustr =
        Ustr::from("trane::guitar::fretboard_exploration::major_scale");
}

pub fn course_builder() -> Result<CourseBuilder> {
    let scale_course = FretboardExplorationCourse {
        course_id: *COURSE_ID,
        dependencies: vec![],
        directory_name: "fretboard_major_scale".to_string(),
        scale: ScaleType::Major,
        note_alias: None,
        tuning: None,
    };
    scale_course.course_builder()
}
