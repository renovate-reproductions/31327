// This file is part of the uutils procps package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use uu_pgrep::process::{ProcessInformation, RunState};

pub(crate) fn pickers(fields: &[String]) -> Vec<Box<dyn Fn(usize) -> String>> {
    fields
        .iter()
        .map(|field| match field.as_str() {
            "PID" => helper(pid),
            "USER" => helper(user),
            "PR" => helper(pr),
            "RES" => helper(res),
            "SHR" => helper(shr),
            "S" => helper(s),
            "%CPU" => helper(cpu),
            "TIME+" => helper(time_plus),
            "%MEM" => helper(mem),
            "COMMAND" => helper(command),
            _ => helper(todo),
        })
        .collect()
}

#[inline]
fn helper(f: impl Fn(usize) -> String + 'static) -> Box<dyn Fn(usize) -> String> {
    Box::new(f)
}

fn todo(_pid: usize) -> String {
    "TODO".into()
}

fn cpu(_pid: usize) -> String {
    "TODO".into()
}

fn pid(pid: usize) -> String {
    pid.to_string()
}

fn user(_pid: usize) -> String {
    "TODO".into()
}

fn pr(_pid: usize) -> String {
    "TODO".into()
}

fn res(_pid: usize) -> String {
    "TODO".into()
}

fn shr(_pid: usize) -> String {
    "TODO".into()
}

fn s(pid: usize) -> String {
    extractor(pid, |mut proc| {
        proc.run_state().unwrap_or(RunState::Stopped).to_string()
    })
}

fn time_plus(_pid: usize) -> String {
    "TODO".into()
}

fn mem(_pid: usize) -> String {
    "TODO".into()
}

fn command(pid: usize) -> String {
    extractor(pid, |mut proc| proc.status()["Name"].clone())
}

/// If cannot constructing [ProcessInformation], it will return "?"
fn extractor<F>(pid: usize, mut f: F) -> String
where
    F: FnMut(ProcessInformation) -> String,
{
    match ProcessInformation::try_new(format!("/proc/{}/", pid).into()) {
        Ok(proc) => f(proc),
        Err(_) => "?".into(),
    }
}
