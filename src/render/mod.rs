use crate::{
    config,
    protocol::rusty::belt::{self, segment_value::Segment},
};
use humansize::{FormatSize, FormatSizeOptions, WINDOWS};

use lazy_static::lazy_static;

lazy_static! {
    static ref CUSTOM_FORMAT: FormatSizeOptions =
        FormatSizeOptions::from(WINDOWS).space_after_value(false);
}

pub fn render_response(response: belt::Response, config: &config::Segment) -> String {
    let default_color = "default".to_string();
    let mut result = Vec::new();
    let mut fg_color_palet = config.fg_palet.iter().cycle();

    if let Some(r) = response.client_response {
        match r {
            belt::response::ClientResponse::Cli(cli) => {
                for opt_segment in cli.values.iter() {
                    let some_result = if let Some(segment_value) = &opt_segment.segment {
                        match segment_value {
                            Segment::ConnectedVpns(vpns) => render_vpn(vpns),
                            Segment::Cpu(cpu) => render_cpu(cpu),
                            Segment::Disk(disk) => render_disk(disk),
                            Segment::LoadAverage(la) => render_load_average(la),
                            Segment::Memory(mem) => render_memory(mem),
                            Segment::MemoryPercent(mem_percents) => {
                                render_memory_percents(mem_percents)
                            }
                            Segment::ShellResult(shell_reuslt) => render_shell_result(shell_reuslt),
                            Segment::Swap(swap) => render_swap(swap),
                            Segment::SwapPercent(swap_percents) => {
                                render_swap_percents(swap_percents)
                            }
                            Segment::TmuxSessionName(session_name) => {
                                render_session_name(session_name)
                            }
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

fn render_session_name(session_name: &belt::TmuxSessionName) -> Option<String> {
    Some(format!("{}", &session_name.session_name))
}

fn render_swap_percents(swap_percents: &belt::SwapPercent) -> Option<String> {
    Some(format!("Swap: {:.0}%", swap_percents.used))
}

fn render_swap(swap: &belt::Swap) -> Option<String> {
    Some(format!(
        "Swap: {}/{}",
        swap.total.format_size(*CUSTOM_FORMAT),
        swap.used.format_size(*CUSTOM_FORMAT)
    ))
}

fn render_shell_result(shell_reuslt: &belt::ShellExecutionResult) -> Option<String> {
    Some(format!("{}", shell_reuslt.std_out))
}

fn render_memory_percents(mem_percents: &belt::MemPercent) -> Option<String> {
    Some(format!("Mem: {:.0}%", mem_percents.used))
}

fn render_memory(mem: &belt::Mem) -> Option<String> {
    Some(format!(
        "Mem: {}/{}/{}",
        mem.total.format_size(*CUSTOM_FORMAT),
        mem.available.format_size(*CUSTOM_FORMAT),
        mem.used.format_size(*CUSTOM_FORMAT)
    ))
}

fn render_load_average(la: &belt::LoadAverage) -> Option<String> {
    Some(format!(
        "LA: {:.2}/{:.2}/{:.2}",
        la.one, la.five, la.fifteen
    ))
}

fn render_disk(disk: &belt::Disk) -> Option<String> {
    Some(format!(
        "{}: {}/{}",
        disk.mount_point,
        disk.total_space_b.format_size(*CUSTOM_FORMAT),
        disk.available_space_b.format_size(*CUSTOM_FORMAT)
    ))
}

fn render_vpn(vpns: &belt::ConnectedVpNs) -> Option<String> {
    if vpns.aliases.is_empty() {
        Some(format!("No VPNs!"))
    } else {
        Some(format!("VPN: {}", vpns.aliases.join(", ")))
    }
}

fn render_cpu(cpu: &belt::Cpu) -> Option<String> {
    Some(format!("CPU: {:.0}%", cpu.consumption))
}
