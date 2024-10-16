use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiPermissions {
    pub connect: bool,
    pub host: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiKey {
    key: String,

    #[serde(default = "ApiPermissions::connect", flatten)]
    permissions: ApiPermissions,
}

impl Default for ApiPermissions {
    fn default() -> Self {
        Self::none()
    }
}

impl ApiPermissions {
    pub const fn none() -> Self {
        Self {
            connect: false,
            host: false,
        }
    }

    pub const fn connect() -> Self {
        Self {
            connect: true,
            host: false,
        }
    }

    pub const fn host() -> Self {
        Self {
            connect: false,
            host: true,
        }
    }

    pub const fn all() -> Self {
        Self {
            connect: true,
            host: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiAccessPolicy {
    pub restrict_connect: bool,
    pub restrict_host: bool,
}

impl Default for ApiAccessPolicy {
    fn default() -> Self {
        Self {
            restrict_connect: true,
            restrict_host: true,
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default)]
pub struct ApiAccessConfig {
    pub policy: ApiAccessPolicy,
    pub keys: Vec<ApiKey>,
}

pub struct ApiAccessManager {
    config: ApiAccessConfig,
}

impl ApiAccessManager {
    pub fn new(config: ApiAccessConfig) -> Self {
        Self { config }
    }

    pub fn get_permissions(&self, key: Option<&str>) -> ApiPermissions {
        let default_perms = ApiPermissions {
            connect: !self.config.policy.restrict_connect,
            host: !self.config.policy.restrict_host,
        };

        let Some(key) = key else {
            return default_perms;
        };

        let Some(key_config) = self.config.keys.iter().find(|k| k.key == key) else {
            return default_perms;
        };

        ApiPermissions {
            connect: !self.config.policy.restrict_connect || key_config.permissions.connect,
            host: !self.config.policy.restrict_host || key_config.permissions.host,
        }
    }
}
