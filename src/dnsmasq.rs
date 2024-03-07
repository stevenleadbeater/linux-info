//! get array with various metrics for DNS and DHCP

use std::sync::Arc;
use std::time::Duration;
use dbus::blocking::{Connection, Proxy};
use dbus::{Error, Path};
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

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
        path: impl Into<Path<'a>>
    ) -> Proxy<'a, &'b Connection> {
        self.conn.with_proxy(DBUS_NAME, path, TIMEOUT)
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
    pub fn metrics(&self) -> Result<Vec<String>, Error> {
        self.dbus.proxy(DBUS_PATH).method_call(DBUS_NAME, "GetMetrics", ())
            .and_then(|r: (Vec<String>, )| Ok(r.0, ))
    }
}
