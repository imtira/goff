mod error;
mod serialize;

use error::Error;

pub fn from_str<'d, T>(inp: &'d str) -> Result<T, Error>
where T: serialize::Deserialize<'de>,
{

}

// Tests
#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  #[derive(Debug, Clone)]
  struct Simple {
    server: String,
    use_proxy: bool,
    timeout: u8,
    proxy: Option<String>,
    developer: Developer,
    server_info: ServerInfo,
  }

  #[derive(Debug, Clone)]
  struct Developer {
    revision: f32,
    license: String,
    work_days: Vec<String>,
    hours_worked: HashMap<String, f32>,
  }

  #[derive(Debug, Clone)]
  struct ServerInfo {
    http_cache: Server,
    seedbox: Server,
  }

  #[derive(Debug, Clone)]
  struct Server {
     ip: String,
     supports_ipv6: bool,
     bandwidth_limit: Option<u32>,
     cpus: u8,
     location: String,
  }

  #[test]
  fn simple() {
    assert_eq!(
      Simple {
        server: "example.com".to_string(),
        use_proxy: false,
        timeout: 5,
        proxy: None,
        developer: Developer {
          revision: 6.66,
          license: "
As long as you retain this notice you can do whatever you want with this stuff. If we meet some day,
and you think this stuff is worth it, you can buy me a beer in return.
          ".to_string(),
          work_days: vec!["Monday".to_string(),
                      "Tuesday".to_string(),
                      "Wednesday".to_string()],
          hours_worked:
            [("Monday".to_string(), 8.),
             ("Tuesday".to_string(), 7.5),
             ("Wednesday".to_string(), 7.5)]
            .iter().cloned().collect(),
        },
        server_info: ServerInfo {
          http_cache: Server {
            ip: "100.100.100.100".to_string(),
            supports_ipv6: true,
            bandwidth_limit: None,
            cpus: 8,
            location: "us-east-2".to_string(),
          },
          seedbox: Server {
            ip: "200.200.200.200".to_string(),
            supports_ipv6: false,
            bandwidth_limit: None,
            cpus: 4,
            location: "us-east-1".to_string(),
          }
        }
      },
      super::from_str(simple_str));
  }
}