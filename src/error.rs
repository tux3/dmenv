use colored::*;

#[derive(Debug)]
pub enum Error {
    ReadError {
        path: std::path::PathBuf,
        io_error: std::io::Error,
    },
    WriteError {
        path: std::path::PathBuf,
        io_error: std::io::Error,
    },

    ProcessWaitError {
        io_error: std::io::Error,
    },
    ProcessOutError {
        io_error: std::io::Error,
    },

    MissingSetupPy {},
    MissingLock {},
    MissingVenv {
        path: std::path::PathBuf,
    },

    FileExists {
        path: std::path::PathBuf,
    },

    Other {
        message: String,
    },

    MalformedLock {
        line: usize,
        details: String,
    },

    NothingToBump {
        name: String,
    },

    MultipleBumps {
        name: String,
    },

    MultipleValues {
        key: String,
    },

    MalformedSetupCfg {
        details: String,
    },

    SectionNotFound {
        name: String,
    },

    KeyNotFound {
        name: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Error::Other { message } => message.to_string(),

            Error::ReadError { path, io_error } => {
                format!("could not read {}: {}", path.to_string_lossy(), io_error)
            }
            Error::WriteError { path, io_error } => {
                format!("could not write {}: {}", path.to_string_lossy(), io_error)
            }

            Error::ProcessWaitError { io_error } => {
                format!("could not wait for process: {}", io_error)
            }
            Error::ProcessOutError { io_error } => {
                format!("could not get process output: {}", io_error)
            }

            Error::MissingSetupPy {} => {
                "setup.py not found.\n You may want to run `dmenv init` now".to_string()
            }
            Error::MissingLock {} => {
                "requirements.lock not found.\n You may want to run `dmenv lock` now".to_string()
            }
            Error::MissingVenv { path } => {
                let mut message = format!(
                    "virtualenv in {} does not exist\n",
                    path.to_string_lossy().bold()
                );
                message.push_str("Please run `dmenv lock` or `dmenv install` to create it");
                message
            }

            Error::FileExists { path } => format!("{} already exist", path.to_string_lossy()),

            Error::MalformedLock { line, details } => {
                format!("malformed lock at line {}\n:{}", line, details)
            }
            Error::NothingToBump { name } => format!("'{}' not found in lock", name),
            Error::MultipleBumps { name } => {
                format!("multiple matches found for '{}' in lock", name)
            }

            Error::MalformedSetupCfg { details } => format!("malformed setup.cfg: {}", details),
            Error::SectionNotFound { name } => format!("section '{}' not found", name),
            Error::KeyNotFound { name } => format!("key '{}' not found", name),
            Error::MultipleValues { key } => format!("multiple values found for key '{}'", key),
        };
        write!(f, "{}", message)
    }
}
