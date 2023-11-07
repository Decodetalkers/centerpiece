use crate::plugin::utils::Plugin;
use anyhow::Context;
use sysinfo::{DiskExt, SystemExt};

pub struct DisksPlugin {
    sysinfo: sysinfo::System,
    entries: Vec<crate::model::Entry>,
}

impl Plugin for DisksPlugin {
    fn id() -> &'static str {
        return "disks";
    }

    fn priority() -> u32 {
        return 12;
    }

    fn title() -> &'static str {
        return "󱛟 Disks";
    }

    fn update_timeout() -> Option<std::time::Duration> {
        return Some(std::time::Duration::from_secs(2));
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.sysinfo.refresh_all();
        self.entries.clear();

        for disk in self.sysinfo.disks() {
            let mount_point = disk
                .mount_point()
                .to_str()
                .context("Unable to convert mount point path to string.")?
                .to_string();

            let used_space = disk.total_space() - disk.available_space();
            let perentage_used = 100 * used_space / disk.total_space();
            let total_space_in_gb = disk.total_space() as f64 / 10_f64.powf(9.);
            let used_space_in_gb = used_space as f64 / 10_f64.powf(9.);

            let title = format!(
                "{} {}% ({:.2}gb / {:.2}gb)",
                &mount_point, perentage_used, used_space_in_gb, total_space_in_gb
            );

            self.entries.push(crate::model::Entry {
                id: mount_point,
                title,
                action: String::from(""),
                meta: String::from("Resource Monitor Disks"),
                command: None,
            });
        }

        return Ok(());
    }

    fn new() -> Self {
        return Self {
            sysinfo: sysinfo::System::new_all(),
            entries: vec![],
        };
    }
}
