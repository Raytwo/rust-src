use crate::ffi::{CString, OsStr};
use crate::fmt;
use crate::io;
use crate::sys::fs::File;
use crate::path::Path;
use crate::sys::pipe::AnonPipe;
use crate::sys::{unsupported, Void};
use crate::sys_common::process::{CommandEnv, CommandEnvs};
use crate::num::NonZeroI32;
use crate::convert::TryInto;
use core::ffi::NonZero_c_int;

pub use crate::ffi::OsString as EnvKey;

use libc::c_int;


////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

pub struct Command {
    env: CommandEnv,
}

// passed back to std::process with the pipes connected to the child, if any
// were requested
pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
}

impl Command {
    pub fn new(_program: &OsStr) -> Command {
        Command { env: Default::default() }
    }

    pub fn arg(&mut self, _arg: &OsStr) {}

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn cwd(&mut self, _dir: &OsStr) {}

    pub fn stdin(&mut self, _stdin: Stdio) {}

    pub fn stdout(&mut self, _stdout: Stdio) {}

    pub fn stderr(&mut self, _stderr: Stdio) {}

    pub fn saw_nul(&self) -> bool {
        unimplemented!()
    }

    pub fn get_program(&self) -> &OsStr {
        unimplemented!()
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        unimplemented!()
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        unimplemented!()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        unimplemented!()
    }

    pub fn spawn(
        &mut self,
        _default: Stdio,
        _needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        unsupported()
    }
}

impl From<AnonPipe> for Stdio {
    fn from(pipe: AnonPipe) -> Stdio {
        pipe.diverge()
    }
}

impl From<File> for Stdio {
    fn from(file: File) -> Stdio {
        file.diverge()
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub struct ExitStatus(Void);

impl ExitStatus {
    pub fn success(&self) -> bool {
        match self.0 {}
    }

    pub fn code(&self) -> Option<i32> {
        match self.0 {}
    }

    #[allow(unreachable_code)]
    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        // This assumes that WIFEXITED(status) && WEXITSTATUS==0 corresponds to status==0.  This is
        // true on all actual versios of Unix, is widely assumed, and is specified in SuS
        // https://pubs.opengroup.org/onlinepubs/9699919799/functions/wait.html .  If it is not
        // true for a platform pretending to be Unix, the tests (our doctests, and also
        // procsss_unix/tests.rs) will spot it.  `ExitStatusError::code` assumes this too.
        //match NonZero_c_int::try_from(unreachable!()) {
        //    /* was nonzero */ Ok(failure) => Err(ExitStatusError(failure)),
        //    /* was zero, couldn't convert */ Err(_) => Ok(()),
        //}
        unreachable!()
    }
}

impl Clone for ExitStatus {
    fn clone(&self) -> ExitStatus {
        match self.0 {}
    }
}

impl Copy for ExitStatus {}

impl PartialEq for ExitStatus {
    fn eq(&self, _other: &ExitStatus) -> bool {
        match self.0 {}
    }
}

impl Eq for ExitStatus {}

impl fmt::Debug for ExitStatus {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatusError(NonZero_c_int);

impl From<ExitStatusError> for ExitStatus {
    #[allow(unreachable_code)]
    fn from(_: ExitStatusError) -> ExitStatus {
        ExitStatus(panic!("Exit status conversion not possible on switch"))
    }
}

/// Converts a raw `c_int` to a type-safe `ExitStatus` by wrapping it without copying.
impl From<c_int> for ExitStatus {
    #[allow(unreachable_code)]
    fn from(_: c_int) -> ExitStatus {
        ExitStatus(unreachable!())
    }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZeroI32> {
        Some(1.try_into().unwrap())
        //ExitStatus(self.0.into()).code().map(|st| st.try_into().unwrap())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode(u8);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0 as _);
    pub const FAILURE: ExitCode = ExitCode(-1 as _);

    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        Self(code)
    }
}

pub struct Process(Void);

impl Process {
    pub fn id(&self) -> u32 {
        match self.0 {}
    }

    pub fn kill(&mut self) -> io::Result<()> {
        match self.0 {}
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        match self.0 {}
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        match self.0 {}
    }
}

pub struct CommandArgs<'a> {
    iter: crate::slice::Iter<'a, CString>,
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<&'a OsStr> {
        unimplemented!();
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!();
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {
    fn len(&self) -> usize {
        unimplemented!();
    }
    fn is_empty(&self) -> bool {
        unimplemented!();
    }
}

#[allow(unused_variables)]
impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!();
    }
}
