use crate::{Day, Error};

pub struct Day20;
impl Day20 {
    pub fn new() -> Self {
        Self
    }

    fn find_allowed_ips(input: &str) -> Vec<(u32, u32)> {
        let mut allowed_ips = vec![(0, u32::MAX)];

        for blocked_ip in Self::parse_input(input) {
            let mut i = 0;
            while i < allowed_ips.len() {
                let allowed_ip = allowed_ips[i];
                if blocked_ip.0 <= allowed_ip.0 && allowed_ip.1 <= blocked_ip.1 {
                    allowed_ips.swap_remove(i);
                } else if allowed_ip.0 < blocked_ip.0 && blocked_ip.0 <= allowed_ip.1 {
                    allowed_ips.push((blocked_ip.0, allowed_ip.1));
                    allowed_ips[i] = (allowed_ip.0, blocked_ip.0 - 1);
                    i += 1;
                } else if allowed_ip.0 <= blocked_ip.1 && blocked_ip.1 < allowed_ip.1 {
                    allowed_ips[i] = (blocked_ip.1 + 1, allowed_ip.1);
                    i += 1;
                } else if blocked_ip.1 == allowed_ip.1 {
                    allowed_ips.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        allowed_ips.sort_by_key(|(start, _)| *start);
        allowed_ips
    }

    fn parse_input(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
        input.lines().map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            (start, end)
        })
    }
}
impl Day for Day20 {
    fn id(&self) -> usize {
        20
    }

    fn title(&self) -> &str {
        "Firewall Rules"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let allowed_ips = Self::find_allowed_ips(input);
        Ok(allowed_ips[0].0.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut allowed_count = 0_u32;
        let allowed_ips = Self::find_allowed_ips(input);

        for allowed_ip_range in allowed_ips {
            allowed_count += allowed_ip_range.1 - allowed_ip_range.0 + 1;
        }

        Ok(allowed_count.to_string())
    }
}
