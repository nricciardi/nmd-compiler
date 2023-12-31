pub mod paragraph;
pub mod chapter_builder;

use std::fmt::Display;
use std::sync::Arc;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

pub use self::paragraph::Paragraph;
use crate::compiler::parsable::codex::Codex;
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;
use crate::compiler::parsable::{codex::parsing_rule::parsing_result::{ParsingError, ParsingOutcome}, Parsable};


pub struct Chapter {
    heading: String,
    paragraphs: Option<Vec<Paragraph>>

    // TODO: maybe in another version
    /* subchapters: Option<Vec<Arc<Chapter>>>,
    superchapter: Option<Arc<Chapter>> */
}

impl Chapter {

    pub fn new(heading: String, paragraphs: Option<Vec<Paragraph>>) -> Self {
        Self {
            heading,
            paragraphs
        }
    }

    pub fn heading(&self) -> &String {
        &self.heading
    }

    pub fn set_heading(&mut self, heading: &String) -> () {
        self.heading = heading.clone()
    }

    pub fn paragraphs(&self) -> &Option<Vec<Paragraph>> {
        &self.paragraphs
    }

    pub fn n_paragraphs(&self) -> usize {
        if let Some(paragraphs) = &self.paragraphs {
            return paragraphs.len()
        }

        0
    }
}

impl Clone for Chapter {
    fn clone(&self) -> Self {
        Self { heading: self.heading.clone(), paragraphs: self.paragraphs.clone() }
    }
}


impl Parsable for Chapter {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        self.heading = codex.parse(&self.heading, Arc::clone(&parsing_configuration))?.parsed_content();

        if let Some(paragraphs) = &mut self.paragraphs {
            let maybe_failed = paragraphs.par_iter_mut().map(|paragraph| {
                paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
            }).find_any(|result| result.is_err());

            if let Some(result) = maybe_failed {
                return result
            }
        }

        Ok(())
    }
}


impl Display for Chapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut s: String = String::from(&self.heading);

        if let Some(paragraphs) = &self.paragraphs {
            for paragraph in paragraphs {
                s.push_str(paragraph.to_string().as_str());
            }
        }

        write!(f, "{}", s)
    }
}

/* impl ToString for Chapter {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
} */