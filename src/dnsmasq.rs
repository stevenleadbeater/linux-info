//! get array with various metrics for DNS and DHCP

use crate::util::read_to_string_mut;

use std::{fs, io};
use std::path::Path;

/// Read uptime information from /uk/org/thekelleys/dnsmasq/GetMetrics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metrics {
    raw: String
}

impl Metrics {
    fn path() -> &'static Path {
        Path::new("/uk/org/thekelleys/dnsmasq/GetMetrics")
    }

    #[cfg(test)]
    fn from_string(raw: String) -> Self {
        Self {raw}
    }

    /// Reads metrics from /uk/org/thekelleys/dnsmasq/GetMetrics.
    pub fn read() -> io::Result<Self> {
        Ok(Self {
            raw: fs::read_to_string(Self::path())?
        })
    }

    /// Reloads information without allocating.
    pub fn reload(&mut self) -> io::Result<()> {
        read_to_string_mut(Self::path(), &mut self.raw)
    }

    /// Main method to get metrics values. Returns every entry.
    pub fn all_infos<'a>(&'a self) -> impl Iterator<Item=String> + 'a {
        self.raw.split(' ')
            .filter_map(|v: &str| v.trim().parse().ok())
            .map(|v| v.to_string())
    }


}
