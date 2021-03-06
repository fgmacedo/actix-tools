#[cfg(feature = "actix")]
extern crate actix;
#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(feature = "config")]
extern crate config;
#[cfg(feature = "diesel")]
extern crate diesel as diesel_core;
#[cfg(feature = "env_logger")]
extern crate env_logger;
#[cfg(feature = "influx_db_client")]
extern crate influx_db_client;
extern crate num_cpus;
#[macro_use]
extern crate log;
#[cfg(feature = "r2d2")]
extern crate r2d2;
#[cfg(feature = "r2d2_redis")]
extern crate r2d2_redis;
#[cfg(feature = "redis")]
extern crate redis as redis_client;
#[cfg(feature = "rumqtt")]
extern crate rumqtt;
#[cfg(feature = "sentry")]
#[macro_use]
pub extern crate sentry as sentry_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate url;

#[cfg(feature = "config")]
pub mod settings;

#[cfg(feature = "diesel_actors")]
pub mod diesel;

#[cfg(feature = "influx_actors")]
pub mod influx;

#[cfg(feature = "json_logs")]
pub mod logs;

#[cfg(feature = "mqtt_actors")]
pub mod mqtt;

#[cfg(feature = "redis_actors")]
pub mod redis;

#[cfg(feature = "sentry")]
pub mod sentry;

pub mod utils;
