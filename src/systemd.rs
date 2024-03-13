//! start and stop systenmd services

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use dbus::blocking::{Connection, Proxy};
use dbus::{Error, Path};

const DBUS_NAME: &str = "org.freedesktop.systemd1";
const DBUS_PATH: &str = "/org/freedesktop/systemd1";
const MANAGER_INTERFACE_NAME: &str = "org.freedesktop.systemd1.Manager";
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
pub struct SystemD {
    dbus: Dbus
}

impl SystemD {
    pub fn connect() -> Result<Self, Error> {
        Dbus::connect()
            .map(|dbus| Self { dbus })
    }
    pub fn start(&self, unit_name: String) -> Result<HashMap<String, u32>, Error> {
        self.dbus.proxy().method_call(MANAGER_INTERFACE_NAME, "StartUnit", (unit_name.as_str(), ))
            .and_then(|r: (HashMap<String, u32>, )| Ok(r.0, ))
    }
    pub fn stop(&self, unit_name: String) -> Result<HashMap<String, u32>, Error> {
        self.dbus.proxy().method_call(MANAGER_INTERFACE_NAME, "StartUnit", (unit_name.as_str(), ))
            .and_then(|r: (HashMap<String, u32>, )| Ok(r.0, ))
    }
}
