// Copyright 2015-2018 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and 
// limitations under the License.

extern crate wascap_guest as guest;

use guest::prelude::*;

call_handler!(handle_call);

pub fn handle_call(ctx: &CapabilitiesContext, cmd: &Command) -> Result<Event> {
    match cmd.payload {
        Some(ref p) => match p.type_url.as_ref() {
            http::TYPE_URL_HTTP_REQUEST => hello_world(ctx, p.value.as_slice()),
            core::TYPE_URL_HEALTH_REQUEST => Ok(Event::success()),
            _ => Ok(Event::bad_dispatch(&p.type_url)),
        },
        None => Ok(http::Response::bad_request().as_event(true, None)),
    }
}

fn hello_world(
   _ctx: &CapabilitiesContext,
   _payload: impl Into<http::Request>) -> Result<Event> {
    Ok(http::Response::ok().as_event(true, None))
}