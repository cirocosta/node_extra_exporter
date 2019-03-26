extern crate libc;

use std::fs;
use std::io;
use std::io::Error;
use std::result::Result;
use std::string::ToString;

lazy_static! {
    /// The number of clock ticks per second
    static ref CLK_TCK: usize = { unsafe { libc::sysconf(libc::_SC_CLK_TCK) as usize } };

    /// The number of processors currently online (available)
    static ref NPROCESSORS_ONLN: usize = {  unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as usize } };
}

pub struct Stat {
    total_running: usize,
    total_waiting: usize,
    timeslices: usize,
}

const SCHEDSTAT_DESCRIPTIONS: &'static [u8] = b"# ---
# HELP node_schedstat_running_seconds_total
# HELP node_schedstat_waiting_seconds_total
# HELP node_schedstat_seconds_total
";

#[inline]
pub fn jiffies_to_seconds(jiffies: usize) -> usize {
    jiffies / *CLK_TCK
}

pub fn processor_count() -> usize {
    *NPROCESSORS_ONLN
}

impl Stat {
    fn to_prometheus_samples(&self, cpu: usize) -> String {
        let mut text = String::with_capacity(3);

        text.push_str(&to_prometheus_sample(
            "node_schedstat_running_seconds_total",
            cpu,
            jiffies_to_seconds(self.total_running),
        ));

        text.push_str(&to_prometheus_sample(
            "node_schedstat_waiting_seconds_total",
            cpu,
            jiffies_to_seconds(self.total_waiting),
        ));

        text.push_str(&to_prometheus_sample(
            "node_schedstat_seconds_total",
            cpu,
            jiffies_to_seconds(self.timeslices),
        ));

        text
    }
}

fn to_prometheus_sample(metric_name: &str, cpu: usize, value: usize) -> String {
    let mut text = String::with_capacity(10);

    text.push_str(metric_name);
    text.push_str(r#"{cpu=""#);
    text.push_str(&cpu.to_string());
    text.push_str(r#""} "#);
    text.push_str(&value.to_string());
    text.push('\n');

    text
}

/// Retrieves a vector of `Stat` structs after parsing
/// the schedstat file from a particular `filepath`.
///
pub fn collect_system_schedstat(filepath: &str) -> Result<Vec<Stat>, Error> {
    let contents = fs::read_to_string(filepath)?;

    Ok(parse_schedstat(&contents))
}

/// Parses a system-wise `schedstat` file, retrieving `cpu` contents
/// from it.
///
/// ```txt
///
///           /proc/schedstat
///        .-----------------------
///        | version 15
///        | timestamp 4302079952
///    0   | cpu0 0 0 0 0 0 0 410561491017 35532088897398 36598258  ---> parsed
///        | domain0 7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
///    1   | cpu1 0 0 0 0 0 0 413817093364 40708332624843 36611731  ---> parsed
///        | domain0 7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
///    2   | cpu2 0 0 0 0 0 0 427904745414 37680886821833 36642621  ---> parsed
///        | domain0 7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
///    EOF *---------------------------
///
/// ```
///
fn parse_schedstat(contents: &str) -> Vec<Stat> {
    let mut v: Vec<Stat> = Vec::new();

    // [cc]: instead, `reduce` it?
    for line in contents.lines() {
        if !line.starts_with("cpu") {
            continue;
        }

        v.push(parse_cpu(line));
    }

    v
}

/// Parses `/proc/schedstat`'s cpu lines.
///
/// Panics if `cpu` line doesn't match the expectations.
/// See [`schedstat` docs][schedstat_docs].
///
/// ```txt
///
///       
///     [ 0  1 2 3 4 5 6      7             8           9   ]
///     cpu0 0 0 0 0 0 0 383568852856 35528196627683 36508380
///       |                   |             |           |
///       |                   |             |           number of timeslices run
///       |                   |             |           on this cpu
///       |                   |             |           (timeslices)
///       |                   |             |
///       |                   |             *-- time spent waiting to run
///       |                   |                 (total_waiting)
///       |                   |             
///       |                   *-- all time spent running by tasks on the processor
///       |                       (total_running)
///       |
///       *---------------------------- logical cpu identification (ignored)
///
/// ```
///
/// [schedstat_docs]: https://www.kernel.org/doc/Documentation/scheduler/sched-stats.txt
///
fn parse_cpu(line: &str) -> Stat {
    let fields: Vec<&str> = line.split(" ").collect();

    assert_eq!(fields.len(), 10);

    Stat {
        total_running: fields[7].parse::<usize>().unwrap(),
        total_waiting: fields[8].parse::<usize>().unwrap(),
        timeslices: fields[9].parse::<usize>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_parse_cpu_empty() {
        let _stat = parse_cpu("");
    }

    #[test]
    #[should_panic(expected = "InvalidDigit")]
    fn test_parse_cpu_with_non_numbers() {
        let _stat = parse_cpu("cpu0 0 0 0 0 0 0 a b c");
    }

    #[test]
    fn test_parse_cpu_correct() {
        let stat = parse_cpu("cpu0 0 0 0 0 0 0 123 456 789");

        assert_eq!(stat.total_running, 123);
        assert_eq!(stat.total_waiting, 456);
        assert_eq!(stat.timeslices, 789);
    }

    #[test]
    fn test_parse_schedstat_empty() {
        let stats = parse_schedstat("");
        assert_eq!(stats.len(), 0);
    }

    #[test]
    fn test_parse_schedstat_no_cpu_lines() {
        let stats = parse_schedstat(
            "version 15
timestamp 12323
foo
bar",
        );
        assert_eq!(stats.len(), 0);
    }

    #[test]
    fn test_parse_schedstat_two_cpus() {
        let stats = parse_schedstat(
            "version 15
timestamp 12323
cpu0 0 0 0 0 0 0 410561491017 35532088897398 36598258
domain a 1 231312
cpu1 0 0 0 0 0 0 413817093364 40708332624843 36611731
bar",
        );

        assert_eq!(stats.len(), 2);
        assert_eq!(stats[0].total_running, 410561491017);
        assert_eq!(stats[1].total_running, 413817093364);
    }

    #[test]
    fn test_stat_to_prometheus_samples() {
        let stat = Stat {
            total_running: 123,
            total_waiting: 456,
            timeslices: 789,
        };

        assert_eq!(
            stat.to_prometheus_samples(1),
            "node_schedstat_running_seconds_total{cpu=\"1\"} 1
node_schedstat_waiting_seconds_total{cpu=\"1\"} 4
node_schedstat_seconds_total{cpu=\"1\"} 7
",
        );
    }

    #[test]
    fn test_to_prometheus_sample() {
        assert_eq!(
            to_prometheus_sample("node_schedstat_running_seconds_total", 1, 123),
            "node_schedstat_running_seconds_total{cpu=\"1\"} 123\n",
        );
    }
}
