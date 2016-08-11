/// Modifiers which can be injected by the application logic to change the state

use std::collections::HashMap;

use ::api;
use ::api::optional::Optional;

/// `StatusModifier`s are used to modify the status
pub trait StatusModifier: Send + Sync {
    /// Called after all registered sensors are read
    fn modify(&self, status: &mut api::Status);
}

/// This modifier updates the opening state based on the
/// people now present sensor.
pub struct StateFromPeopleNowPresent;

impl StatusModifier for StateFromPeopleNowPresent {
    fn modify(&self, status: &mut api::Status) {
        // Update state depending on number of people present
        let people_now_present: Option<u64> = status.sensors.as_ref()
            .and_then(|sensors| sensors.people_now_present.as_ref())
            .map(|people_now_present| people_now_present[0].value)
            .into();
        if let Some(count) = people_now_present {
            status.state.open = Some(count > 0);
            if count == 1 {
                status.state.message = Optional::Value(format!("{} person here right now", count));
            } else if count > 1 {
                status.state.message = Optional::Value(format!("{} people here right now", count));
            }
        }
    }
}

/// This modifier adds internal version information to the output.
pub struct LibraryVersions;

impl StatusModifier for LibraryVersions {
    fn modify(&self, status: &mut api::Status) {
        // Add library version information
        let api_version = api::get_version().to_string();
        let server_version = ::get_version().to_string();

        // Create version map if it doesn't exist yet
        if status.ext_versions.is_absent() {
            status.ext_versions = Optional::Value(HashMap::new());
        }

        // Add to map
        // TODO: Simplify this stuff once spaceapi-rs moved to serde
        // and doesn't need Optional anymore.
        if let Optional::Value(ref mut map) = status.ext_versions {
            map.insert("spaceapi-rs".to_string(), api_version);
            map.insert("spaceapi-server-rs".to_string(), server_version);
        }
    }
}
