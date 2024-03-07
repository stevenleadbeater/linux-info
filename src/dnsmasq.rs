//! get array with various metrics for DNS and DHCP

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use dbus::blocking::{Connection, Proxy};
use dbus::{Error, Path};

const DBUS_NAME: &str = "uk.org.thekelleys.dnsmasq";
const DBUS_PATH: &str = "/uk/org/thekelleys/dnsmasq";
const TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone)]
struct Dbus {
    conn: Arc<Connection>
}

impl Dbus {
    fn connect() -> Result<Self, Error> {
        Connection::new_system()
            .map(Arc::new)
            .map(|conn| Self { conn })
    }

    fn proxy<'a, 'b>(
        &'b self,
    ) -> Proxy<'a, &'b Connection> {
        self.conn.with_proxy(DBUS_NAME, DBUS_PATH, TIMEOUT)
    }
}

#[derive(Clone)]
pub struct DNSMasq {
    dbus: Dbus
}

impl DNSMasq {
    pub fn connect() -> Result<Self, Error> {
        Dbus::connect()
            .map(|dbus| Self { dbus })
    }
    pub fn metrics(&self) -> Result<HashMap<String, u32>, Error> {
        self.dbus.proxy().method_call(DBUS_NAME, "GetMetrics", ())
            .and_then(|r: (HashMap<String, u32>, )| Ok(r.0, ))
    }
}
