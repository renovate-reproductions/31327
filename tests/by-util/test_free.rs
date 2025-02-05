// This file is part of the uutils procps package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use pretty_assertions::assert_eq;
use regex::Regex;

use crate::common::util::TestScenario;

// TODO: make tests combineable (e.g. test --total --human)

#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails().code_is(1);
}

#[test]
fn test_no_args() {
    let header_pattern = r"^ {15}total {8}used {8}free {6}shared {2}buff/cache {3}available$";
    let mem_pattern = r"^Mem:( +\d+){6}$";
    let swap_pattern = r"^Swap: ( +\d+){3}$";

    let patterns = vec![
        Regex::new(header_pattern).unwrap(),
        Regex::new(mem_pattern).unwrap(),
        Regex::new(swap_pattern).unwrap(),
    ];

    let binding = new_ucmd!().succeeds();
    let output = binding.stdout_str();
    assert_eq!(output.len(), 207);

    // Check the format for each line output
    let mut lines = output.lines();
    for pattern in patterns {
        assert!(pattern.is_match(lines.next().unwrap()));
    }
}

#[test]
fn test_line() {
    let pattern = Regex::new(r"^SwapUse +\d+ CachUse +\d+ {2}MemUse +\d+ MemFree +\d+\n$").unwrap();

    for arg in ["-L", "--line"] {
        new_ucmd!().arg(arg).succeeds().stdout_matches(&pattern);
    }

    // ensure --line "wins"
    for arg in ["--lohi", "--total", "--committed", "--wide"] {
        new_ucmd!()
            .arg(arg)
            .arg("--line")
            .arg(arg)
            .succeeds()
            .stdout_matches(&pattern);
    }
}

#[test]
fn test_wide() {
    let header_pattern = r"^ {15}total {8}used {8}free {6}shared {5}buffers {7}cache {3}available$";
    let mem_pattern = r"^Mem:( +\d+){7}$";
    let swap_pattern = r"^Swap: ( +\d+){3}$";

    let patterns = vec![
        Regex::new(header_pattern).unwrap(),
        Regex::new(mem_pattern).unwrap(),
        Regex::new(swap_pattern).unwrap(),
    ];

    for arg in ["-w", "--wide"] {
        let binding = new_ucmd!().arg(arg).succeeds();
        let output = binding.stdout_str();

        // The total number of character is always fixed
        assert_eq!(output.len(), 231);

        // Check the format for each line output
        let mut lines = output.lines();
        for pattern in &patterns {
            assert!(pattern.is_match(lines.next().unwrap()));
        }
    }
}

#[test]
fn test_total() {
    for arg in ["-t", "--total"] {
        let result = new_ucmd!().arg(arg).succeeds();
        assert_eq!(result.stdout_str().lines().count(), 4);
        assert!(result
            .stdout_str()
            .lines()
            .last()
            .unwrap()
            .starts_with("Total:"));
    }
}

#[test]
fn test_count() {
    let result = new_ucmd!().args(&["-c", "2", "-s", "0"]).succeeds();
    assert_eq!(result.stdout_str().lines().count(), 7);
}

#[test]
fn test_lohi() {
    for arg in ["-l", "--lohi"] {
        let result = new_ucmd!().arg(arg).succeeds();
        assert_eq!(result.stdout_str().lines().count(), 5);
        let lines = result.stdout_str().lines().collect::<Vec<&str>>();
        assert!(lines[2].starts_with("Low:"));
        assert!(lines[3].starts_with("High:"));
    }
}

#[test]
fn test_committed() {
    for arg in ["-v", "--committed"] {
        let result = new_ucmd!().arg(arg).succeeds();
        assert_eq!(result.stdout_str().lines().count(), 4);
        assert!(result
            .stdout_str()
            .lines()
            .last()
            .unwrap()
            .starts_with("Comm:"));
    }
}
