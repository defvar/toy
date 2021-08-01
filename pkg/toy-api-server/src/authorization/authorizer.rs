use crate::context::Context;
use crate::ApiError;
use toy_api::role::{Role, RESOURCE_ALL, VERB_ALL};

pub fn authorize(ctx: &Context, roles: Option<Vec<Role>>) -> Result<(), ApiError> {
    tracing::trace!("authorize. ctx:{:?}, rules:{:?}", ctx, roles);

    if let Ok(v) = std::env::var("TOY_AUTHORIZATION") {
        if v == "none" {
            tracing::warn!("skip authorization.");
            return Ok(());
        }
    }

    if let Some(roles) = roles {
        for r in roles {
            for r in r.rules() {
                if match_resource(&ctx, r.resources()) && match_verb(&ctx, r.verbs()) {
                    return Ok(());
                }
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
