pub mod chart;
pub mod templater;
pub mod view;

use crate::{
    config,
    config::GetTemplate,
    protocol::rusty::belt::{self, segment_value::Segment},
    render::templater::Templater,
};

use lazy_static::lazy_static;

use self::view::representation::{Disk, Mem, Session, Shell, Swap, CPU, LA, VPN};

lazy_static! {
    static ref TEMPLATER: Templater = Templater::default();
}

pub fn render_response(response: belt::Response, config: &config::Segment) -> String {
    let default_color = "default".to_string();
    let mut result = Vec::new();
    let mut fg_color_palet = config.fg_palet.iter().cycle();
    let segments = &config.parts;

    if let Some(r) = response.client_response {
        match r {
            belt::response::ClientResponse::Cli(cli) => {
                for opt_segment in cli.values.iter() {
                    let segment_id = opt_segment.id as usize;

                    let some_result = if let Some(segment_value) = &opt_segment.segment {
                        if let Some(part) = segments.get(segment_id) {
                            let template = &part.template();
                            match segment_value {
                                Segment::ConnectedVpns(vpns) => render_vpn(vpns, template),
                                Segment::Cpu(cpu) => render_cpu(cpu, template),
                                Segment::Disk(disk) => render_disk(disk, template),
                                Segment::LoadAverage(la) => render_load_average(la, template),
                                Segment::Memory(mem) => render_memory(mem, template),
                                Segment::ShellResult(shell_reuslt) => {
                                    render_shell_result(shell_reuslt, template)
                                }
                                Segment::Swap(swap) => render_swap(swap, template),
                                Segment::TmuxSessionName(session_name) => {
                                    render_session_name(session_name, template)
                                }
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    if let Some(rendered_part) = some_result {
                        let fg_color = fg_color_palet
                            .next()
                            .map(|s| format!("#{}", s))
                            .unwrap_or(default_color.clone());
                        result.push(format!("#[fg={}]{} #[fg=default]", fg_color, rendered_part));
                    }
                }
            }
        }
    }
    result.join(" ")
}

fn render_session_name(session_name: &belt::TmuxSessionName, template_str: &str) -> Option<String> {
    Some(
        TEMPLATER
            .rendere_template(template_str, &Session::from(session_name))
            .unwrap(),
    )
}

fn render_swap(swap: &belt::Swap, template_str: &str) -> Option<String> {
    Some(
        TEMPLATER
            .rendere_template(template_str, &Swap::from(swap))
            .unwrap(),
    )
}

fn render_shell_result(
    shell_reuslt: &belt::ShellExecutionResult,
    template_str: &str,
) -> Option<String> {
    Some(
        TEMPLATER
            .rendere_template(template_str, &Shell::from(shell_reuslt))
            .unwrap(),
    )
}

fn render_memory(mem: &belt::Mem, template_str: &str) -> Option<String> {
    Some(
        TEMPLATER
            .rendere_template(template_str, &Mem::from(mem))
            .unwrap(),
    )
}

fn render_load_average(la: &belt::LoadAverage, template_str: &str) -> Option<String> {
    Some(
        TEMPLATER
            .rendere_template(template_str, &LA::from(la))
            .unwrap(),
    )
}

fn render_disk(disk: &belt::Disk, template_str: &str) -> Option<String> {
    Some(
        TEMPLATER
            .rendere_template(template_str, &Disk::from(disk))
            .unwrap(),
    )
}

fn render_vpn(vpns: &belt::ConnectedVpNs, template_str: &str) -> Option<String> {
    Some(
        TEMPLATER
            .rendere_template(template_str, &VPN::from(vpns))
            .unwrap(),
    )
}

fn render_cpu(cpu: &belt::Cpu, template_str: &str) -> Option<String> {
    Some(
        TEMPLATER
            .rendere_template(template_str, &CPU::from(cpu))
            .unwrap(),
    )
}
