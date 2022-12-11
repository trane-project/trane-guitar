use anyhow::Result;
use lazy_static::lazy_static;
use trane::{course_builder::CourseBuilder, data::music::scales::ScaleType};
use ustr::Ustr;

use crate::fretboard::fretboard_exploration::FretboardExplorationCourse;

lazy_static! {
    pub static ref COURSE_ID: Ustr =
        Ustr::from("trane::guitar::fretboard_exploration::minor_scale");
}

pub fn course_builder() -> Result<CourseBuilder> {
    let scale_course = FretboardExplorationCourse {
        course_id: *COURSE_ID,
        dependencies: vec![],
        directory_name: "fretboard_minor_scale".to_string(),
        scale: ScaleType::Minor,
        note_alias: Some(|note| note.relative_minor()),
        tuning: None,
    };
    scale_course.course_builder()
}
