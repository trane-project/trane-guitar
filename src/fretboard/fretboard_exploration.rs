use anyhow::Result;
use indoc::{formatdoc, indoc};
use std::collections::BTreeMap;
use trane::{
    course_builder::{
        music::{circle_fifths::CircleFifthsCourse, notes::Note, scales::ScaleType, MusicMetadata},
        AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder, TraneMetadata,
    },
    data::{
        BasicAsset, CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType,
        LessonManifestBuilder,
    },
};
use ustr::Ustr;

use crate::AUTHORS;

/// A course to explore a given scale all over the fretboard.
pub struct FretboardExplorationCourse {
    /// The ID of the course.
    pub course_id: Ustr,

    /// The dependencies for this course.
    pub dependencies: Vec<Ustr>,

    /// The name of the directory under which the course will be stored.
    pub directory_name: String,

    /// The scale type that this course is about.
    pub scale: ScaleType,

    /// An optinal function used to generate the name of the note for the lesson. Useful, for
    /// example, to generate a course on the minor scale which follows the circle of fifths.
    pub note_alias: Option<fn(Note) -> Result<Note>>,

    /// An optional vector of notes to represent the tuning of the guitar. If not provided, the
    /// standard tuning will be used.
    pub tuning: Option<Vec<Note>>,
}

impl FretboardExplorationCourse {
    /// Returns the standard tuning. The E string is only returned once since there's no point in
    /// repeating it.
    fn standard_tuning() -> Vec<Note> {
        vec![Note::E, Note::A, Note::D, Note::G, Note::B]
    }

    /// Generates the exercise builders for the lesson with the given scale and note.
    fn generate_exercise_builders(
        course_id: Ustr,
        scale: ScaleType,
        note: Note,
        tuning: Option<Vec<Note>>,
    ) -> Result<Vec<ExerciseBuilder>> {
        let scale_notes = scale.notes(note)?.notes;
        let scale_answer = scale_notes
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let mut builders = vec![];
        let tuning = match tuning {
            None => Self::standard_tuning(),
            Some(tuning) => tuning,
        };

        for guitar_string in tuning {
            builders.push(ExerciseBuilder {
                directory_name: format!("{}_string", guitar_string.to_string()),
                asset_builders: vec![
                    AssetBuilder {
                        file_name: "front.md".to_string(),
                        contents: formatdoc! {"
                           Explore the {} {} scale in the {} string. 
                        ", note.to_string(), scale.to_string(), guitar_string.to_string()},
                    },
                    AssetBuilder {
                        file_name: "back.md".to_string(),
                        contents: formatdoc! {"
                            The notes of the {} {} scale are: {}.
                        ", note.to_string(), scale.to_string(), scale_answer},
                    },
                ],
                manifest_closure: Box::new(move |m| {
                    #[allow(clippy::redundant_clone)]
                    m.clone()
                        .id(format!(
                            "{}::{}::{}_string",
                            course_id,
                            note.to_string(),
                            guitar_string.to_string(),
                        ))
                        .name(format!(
                            "Explore the {} {} scale in the {} string",
                            note.to_string(),
                            scale.to_string(),
                            guitar_string.to_string()
                        ))
                        .clone()
                }),
            })
        }

        Ok(builders)
    }

    /// Generates the course builder for the scale course.
    pub fn course_builder(&self) -> Result<CourseBuilder> {
        let course_id = self.course_id;
        let scale = self.scale;
        let tuning = self.tuning.to_owned();

        let course_generator = CircleFifthsCourse {
            directory_name: self.directory_name.clone(),
            course_manifest: CourseManifest {
                id: self.course_id,
                name: format!("Explore the {} Scale in the fretboard", scale.to_string()),
                dependencies: self.dependencies.clone(),
                description: Some(format!(
                    "Explore the {} scale in all strings in the fretboard for all keys.",
                    scale.to_string()
                )),
                authors: Some(vec![AUTHORS.to_string()]),
                metadata: Some(BTreeMap::from([
                    (TraneMetadata::Skill.to_string(), vec!["music".to_string()]),
                    (
                        MusicMetadata::Instrument.to_string(),
                        vec!["guitar".to_string()],
                    ),
                    (
                        MusicMetadata::MusicalSkill.to_string(),
                        vec!["fretboard".to_string()],
                    ),
                    (
                        MusicMetadata::MusicalConcept.to_string(),
                        vec!["scales".to_string()],
                    ),
                    (
                        MusicMetadata::ScaleType.to_string(),
                        vec![scale.to_string().to_lowercase()],
                    ),
                ])),
                course_material: None,
                course_instructions: Some(BasicAsset::MarkdownAsset {
                    path: "course_instructions.md".to_string(),
                }),
            },
            course_asset_builders: vec![AssetBuilder {
                file_name: "course_instructions.md".to_string(),
                contents: indoc! {"
                        Inspired by an exercise from the book *The Advancing guitarist*.

                        Explore the scale in each individual string without jumping across
                        multiple strings. Explore different fingerings, techniques, dynamics,
                        etc.

                        You can use a vamp or backing track, although they are not provided
                        here.
                    "}
                .to_string(),
            }],
            note_alias: self.note_alias,
            lesson_manifest_template: LessonManifestBuilder::default()
                .course_id(self.course_id)
                .clone(),
            lesson_builder_generator: Box::new(move |note, previous_note| {
                let lesson_id = format!("{}::{}", course_id, note.to_string());

                Ok(LessonBuilder {
                    directory_name: format!("lesson_{}", note.to_ascii_string()),
                    exercise_manifest_template: ExerciseManifestBuilder::default()
                        .course_id(course_id)
                        .lesson_id(lesson_id)
                        .exercise_type(ExerciseType::Procedural)
                        .exercise_asset(ExerciseAsset::FlashcardAsset {
                            front_path: "front.md".to_string(),
                            back_path: "back.md".to_string(),
                        })
                        .clone(),
                    asset_builders: vec![],
                    exercise_builders: Self::generate_exercise_builders(
                        course_id,
                        scale,
                        note,
                        tuning.clone(),
                    )?,
                    manifest_closure: Box::new(move |m| {
                        let deps = match previous_note {
                            None => vec![],
                            Some(previous_note) => {
                                let dep_id = Ustr::from(&format!(
                                    "{}::{}",
                                    course_id,
                                    previous_note.to_string()
                                ));
                                vec![dep_id]
                            }
                        };

                        #[allow(clippy::redundant_clone)]
                        m.clone()
                            .id(format!("{}::{}", course_id, note.to_string()))
                            .name(format!(
                                "Explore the {} {} Scale in the fretboard",
                                note.to_string(),
                                scale.to_string(),
                            ))
                            .description(Some(format!(
                                "Explore the notes of the {} {} scale in the fretboard.",
                                note.to_string(),
                                scale.to_string(),
                            )))
                            .dependencies(deps)
                            .metadata(Some(BTreeMap::from([(
                                MusicMetadata::Key.to_string(),
                                vec![note.to_ascii_string()],
                            )])))
                            .clone()
                    }),
                })
            }),
            extra_lessons_generator: None,
        };
        course_generator.generate_course_builder()
    }
}
