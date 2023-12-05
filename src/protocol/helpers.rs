use super::rusty::belt::{self, segment_value, SegmentValue};
use crate::model::Context;

pub fn get_context(request: &belt::Request) -> Context {
    let mut context = Context::new();

    request.context.iter().for_each(|ctx| match ctx {
        belt::request::Context::Tmux(t) => {
            context.current_directory = Some(t.pwd.clone());
            context.tmux_session_name = Some(t.session_name.clone());
        }
    });
    context
}

pub fn build_cli_response(
    model_states: Vec<segment_value::Segment>,
) -> belt::response::ClientResponse {
    let mut cli_client_resposnse = belt::CliClientResponse::default();
    cli_client_resposnse.values = model_states
        .iter()
        .map(|s| {
            let mut value = SegmentValue::default();
            value.segment = Some(s.clone());
            value
        })
        .collect();
    belt::response::ClientResponse::Cli(cli_client_resposnse)
}
