use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};

mod util;
pub use self::util::parse_error::ParseError;
pub use self::util::datetime::{Date,Time};
pub use self::util::coord::{Compass,RawCoord,RawPosition};

mod records;
pub use self::records::*;


pub struct Task {
    pub declaration: CRecordDeclaration,
    pub turnpoints: Vec<CRecordTurnpoint>,
}

impl Task {
    fn from(declaration: CRecordDeclaration) -> Self {
        Task { declaration, turnpoints: Vec::<CRecordTurnpoint>::new() }
    }
}


/// Closely represents a parsed IGC file, with minimal post-processing
pub struct IGCFile {
    pub filepath: PathBuf,
    pub fixes: Vec<BRecord>,
    pub task: Option<Task>,
}

impl IGCFile {
    fn _new(filepath: &Path) -> Self {
        IGCFile {
            filepath: filepath.to_path_buf(),
            fixes: Vec::<BRecord>::new(),
            task: None
        }
    }

    fn _parse_line(&mut self, line: &str) -> Result<(), ParseError> {
        match line.as_bytes()[0] {
            b'B' => self.fixes.push(BRecord::parse(line)?),
            b'C' => {
                if let Some(ref mut task) = self.task {
                    task.turnpoints.push(CRecordTurnpoint::parse(line)?);
                } else {
                    self.task = Some(Task::from(CRecordDeclaration::parse(line)?));
                }
            },
            _ => ()
        }

        Ok(())
    }

    pub fn parse(filepath: &Path) -> Result<Self, ParseError> {
        let f = match File::open(filepath) {
            Ok(file) => file,
            Err(e) => return Err(ParseError::IOError(e)),
        };

        let mut igc_file = Self::_new(filepath);

        for line in BufReader::new(f).lines() {
            let line_result = match line {
                Ok(line) => igc_file._parse_line(&line[..]),
                Err(e) => Err(ParseError::IOError(e)),
            };

            if let Err(e) = line_result {
                return Err(e)
            }
        }

        Ok(igc_file)
    }
}
