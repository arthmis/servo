/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! The Accessibility actor is responsible for the Accessibility tab in the DevTools page. Right
//! now it is a placeholder for future functionality.

use serde::Serialize;
use serde_json::{Map, Value};

use crate::StreamId;
use crate::actor::{Actor, ActorError, ActorRegistry};
use crate::protocol::ClientRequest;

#[derive(Serialize)]
struct BootstrapState {
    enabled: bool,
}

#[derive(Serialize)]
struct BootstrapReply {
    from: String,
    state: BootstrapState,
}

#[derive(Serialize)]
struct GetSimulatorReply {
    from: String,
    simulator: ActorMsg,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AccessibilityTraits {
    tabbing_order: bool,
}

#[derive(Serialize)]
struct GetTraitsReply {
    from: String,
    traits: AccessibilityTraits,
}

#[derive(Serialize)]
struct ActorMsg {
    actor: String,
}

#[derive(Serialize)]
struct GetWalkerReply {
    from: String,
    walker: ActorMsg,
}

pub struct AccessibilityActor {
    name: String,
}

impl Actor for AccessibilityActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    /// The accesibility actor can handle the following messages:
    ///
    /// - `bootstrap`: It is required but it doesn't do anything yet
    ///
    /// - `getSimulator`: Returns a new Simulator actor
    ///
    /// - `getTraits`: Informs the DevTools client about the configuration of the accessibility actor
    ///
    /// - `getWalker`: Returns a new AccessibleWalker actor (not to be confused with the general
    ///   inspector Walker actor)
    fn handle_message(
        &self,
        request: ClientRequest,
        registry: &ActorRegistry,
        msg_type: &str,
        _msg: &Map<String, Value>,
        _id: StreamId,
    ) -> Result<(), ActorError> {
        match msg_type {
            "bootstrap" => {
                let msg = BootstrapReply {
                    from: self.name(),
                    state: BootstrapState { enabled: false },
                };
                request.reply_final(&msg)?
            },
            "getSimulator" => {
                // TODO: Create actual simulator
                let simulator = registry.new_name("simulator");
                let msg = GetSimulatorReply {
                    from: self.name(),
                    simulator: ActorMsg { actor: simulator },
                };
                request.reply_final(&msg)?
            },
            "getTraits" => {
                let msg = GetTraitsReply {
                    from: self.name(),
                    traits: AccessibilityTraits {
                        tabbing_order: true,
                    },
                };
                request.reply_final(&msg)?
            },
            "getWalker" => {
                // TODO: Create actual accessible walker
                let walker = registry.new_name("accesiblewalker");
                let msg = GetWalkerReply {
                    from: self.name(),
                    walker: ActorMsg { actor: walker },
                };
                request.reply_final(&msg)?
            },
            _ => return Err(ActorError::UnrecognizedPacketType),
        };
        Ok(())
    }
}

impl AccessibilityActor {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
