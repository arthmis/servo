/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use base::id::{BrowsingContextId, ServiceWorkerId};
use malloc_size_of_derive::MallocSizeOf;
use serde::{Deserialize, Serialize};
use servo_url::ServoUrl;
use url::Origin;

use crate::policy_container::PolicyContainer;

/// https://html.spec.whatwg.org/multipage/webappapis.html#environment
#[derive(Debug, Clone, MallocSizeOf)]
pub struct Environment {
    pub id: String,
    pub creation_url: ServoUrl,
    pub top_level_creation_url: Option<ServoUrl>,
    #[ignore_malloc_size_of = "because can't implement for Origin, it's in Url crate"]
    pub top_level_origin: Origin,
    pub target_browsing_context: BrowsingContextId,
    pub active_service_worker: Option<ServiceWorkerId>,
    pub execution_ready: bool,
}

/// https://html.spec.whatwg.org/multipage/webappapis.html#environment-settings-object
#[derive(Debug, Clone, MallocSizeOf)]
pub struct EnvironmentSettings {
    pub environment: Environment,
    pub policy_container: PolicyContainer,
    #[ignore_malloc_size_of = "because can't implement for Origin, it's in Url crate"]
    pub origin: Origin,
    pub cross_origin_isolated_capability: bool,
}

/// https://fetch.spec.whatwg.org/#concept-request-client
#[derive(Debug, Clone, MallocSizeOf)]
pub struct RequestClient {
    pub environment_settings: Option<EnvironmentSettings>,
}

impl RequestClient {
    pub fn is_none(&self) -> bool {
        self.environment_settings.is_none()
    }

    pub fn embedder_policy_value(&self) -> EmbedderPolicyValue {
        self.environment_settings.as_ref().unwrap().policy_container.embedder_policy.value
    }

}

/// https://html.spec.whatwg.org/multipage/browsers.html#embedder-policy
#[derive(Clone, Debug, Default, Deserialize, MallocSizeOf, Serialize)]
pub struct EmbedderPolicy {
    pub value: EmbedderPolicyValue,
    pub reporting_endpoint: String,
    pub report_only_value: EmbedderPolicyValue,
    pub report_only_reporting_endpoint: String,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, MallocSizeOf, Serialize, Eq, PartialEq)]
/// https://html.spec.whatwg.org/multipage/browsers.html#embedder-policy-value
pub enum EmbedderPolicyValue {
    #[default]
    UnsafeNone,
    RequireCorp,
    CredentialLess,
}
