use miette::Diagnostic;
use sysexits::ExitCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
    #[error("invalid command")]
    #[diagnostic(code(drone::usage))]
    Clap(#[from] clap::Error),

    #[error("`drone {command}` is not implemented")]
    #[diagnostic(
        code(drone::unimplemented),
        help("the command was recognized, but its behavior has not been built yet")
    )]
    Unimplemented {
        command: &'static str,
        resources: Vec<String>,
    },
}

impl Error {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Clap(_) => ExitCode::Usage,
            Self::Unimplemented { .. } => ExitCode::Unavailable,
        }
    }
}

impl Error {
    pub fn report(self) -> ExitCode {
        if let Self::Clap(error) = &self
            && matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            )
        {
            let _ = error.print();
            return ExitCode::Ok;
        }

        let exit_code = self.exit_code();
        eprintln!("{:?}", miette::Report::new(self));
        exit_code
    }
}
