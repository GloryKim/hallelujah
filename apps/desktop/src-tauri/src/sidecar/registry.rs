/// One entry per sidecar. Generated from packages/contracts/ports.yaml.
/// When adding a new sidecar, run `pnpm generate` to regenerate this file,
/// or add the entry manually following the same pattern.
pub struct SidecarEntry {
    pub id: &'static str,
    pub binary: &'static str,
    pub port: u16,
    pub health_path: &'static str,
}

pub const SIDECARS: &[SidecarEntry] = &[
    SidecarEntry {
        id: "sc-gin",
        binary: "sc-gin",
        port: 7101,
        health_path: "/health",
    },
    SidecarEntry {
        id: "sc-express",
        binary: "sc-express",
        port: 7102,
        health_path: "/health",
    },
    SidecarEntry {
        id: "sc-fastapi",
        binary: "sc-fastapi",
        port: 7103,
        health_path: "/health",
    },
    SidecarEntry {
        id: "sc-nest",
        binary: "sc-nest",
        port: 7104,
        health_path: "/health",
    },
];
