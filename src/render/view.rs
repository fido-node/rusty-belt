pub mod representation {

    use lazy_static::lazy_static;

    use humansize::{FormatSize, FormatSizeOptions, WINDOWS};
    use serde::Serialize;

    use crate::protocol::rusty::belt;

    lazy_static! {
        static ref CUSTOM_FORMAT: FormatSizeOptions =
            FormatSizeOptions::from(WINDOWS).space_after_value(false);
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct Session {
        pub v: String,
    }

    impl From<&belt::TmuxSessionName> for Session {
        fn from(value: &belt::TmuxSessionName) -> Self {
            Session {
                v: value.session_name.clone(),
            }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct VPN {
        pub v: Vec<String>,
    }

    impl From<&belt::ConnectedVpNs> for VPN {
        fn from(value: &belt::ConnectedVpNs) -> Self {
            VPN {
                v: value.aliases.iter().map(|s| s.clone()).collect(),
            }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    struct MemV {
        pub total: String,
        pub available: String,
        pub used: String,
        pub used_percents: String,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct Mem {
        pub v: MemV,
    }

    impl From<&belt::Mem> for Mem {
        fn from(value: &belt::Mem) -> Self {
            let memv = MemV {
                available: value.available.format_size(*CUSTOM_FORMAT),
                total: value.total.format_size(*CUSTOM_FORMAT),
                used: value.used.format_size(*CUSTOM_FORMAT),
                used_percents: format!("{:.0}", ((value.used as f64 / value.total as f64) * 100.0)),
            };
            Mem { v: memv }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    struct LAV {
        pub one: String,
        pub five: String,
        pub fifteen: String,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct LA {
        pub v: LAV,
    }

    impl From<&belt::LoadAverage> for LA {
        fn from(value: &belt::LoadAverage) -> Self {
            let lav = LAV {
                one: format!("{:.2}", value.one),
                five: format!("{:.2}", value.five),
                fifteen: format!("{:.2}", value.fifteen),
            };
            LA { v: lav }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct CPU {
        pub v: String,
    }

    impl From<&belt::Cpu> for CPU {
        fn from(value: &belt::Cpu) -> Self {
            CPU {
                v: format!("{:.0}", value.consumption),
            }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    struct SwapV {
        pub total: String,
        pub used: String,
        pub used_percents: String,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct Swap {
        pub v: SwapV,
    }

    impl From<&belt::Swap> for Swap {
        fn from(value: &belt::Swap) -> Self {
            let swapv = SwapV {
                total: value.total.format_size(*CUSTOM_FORMAT),
                used: value.used.format_size(*CUSTOM_FORMAT),
                used_percents: format!("{:.0}", ((value.used as f64 / value.total as f64) * 100.0)),
            };
            Swap { v: swapv }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    struct DiskV {
        pub mount_point: String,
        pub device_path: String,
        pub available_space: String,
        pub total_space: String,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct Disk {
        pub v: DiskV,
    }

    impl From<&belt::Disk> for Disk {
        fn from(value: &belt::Disk) -> Self {
            let diskv = DiskV {
                mount_point: value.mount_point.clone(),
                device_path: value.device_path.clone(),
                available_space: value.available_space_b.format_size(*CUSTOM_FORMAT),
                total_space: value.total_space_b.format_size(*CUSTOM_FORMAT),
            };
            Disk { v: diskv }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    struct ShellV {
        pub stdout: String,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct Shell {
        pub v: ShellV,
    }

    impl From<&belt::ShellExecutionResult> for Shell {
        fn from(value: &belt::ShellExecutionResult) -> Self {
            let shellv = ShellV {
                stdout: value.stdout.clone(),
            };
            Shell { v: shellv }
        }
    }
}