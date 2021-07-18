use crate::context::Context;
use crate::ApiError;
use toy_api::role::{Rule, RESOURCE_ALL, VERB_ALL};

pub fn authorize(ctx: &Context, rules: Option<Vec<Rule>>) -> Result<(), ApiError> {
    tracing::trace!("authorize. ctx:{:?}, rules:{:?}", ctx, rules);

    if let Ok(v) = std::env::var("TOY_AUTHORIZATION") {
        if v == "none" {
            tracing::warn!("skip authorization.");
            return Ok(());
        }
    }

    if let Some(rules) = rules {
        for r in rules {
            if match_resource(&ctx, r.resources()) && match_verb(&ctx, r.verbs()) {
                return Ok(());
            }
        }
    }
    Err(ApiError::authorization_failed(
        ctx.user().name(),
        ctx.resource(),
        ctx.verb(),
    ))
}

fn match_resource(ctx: &Context, resources: &Vec<String>) -> bool {
    for v in resources {
        if v == RESOURCE_ALL {
            return true;
        }
        if v == ctx.resource() {
            return true;
        }
    }
    false
}

fn match_verb(ctx: &Context, verbs: &Vec<String>) -> bool {
    for v in verbs {
        if v == VERB_ALL {
            return true;
        }
        if v == ctx.verb() {
            return true;
        }
    }
    false
}
