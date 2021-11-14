use super::*;
use std::io::Write;

impl<'api, 'pin> Runner<'api, 'pin> {
    pub fn rename(&mut self, cmd: &SubCommand) {
        match cmd {
            SubCommand::Rename { tags } => self.run(tags),
            _ => unreachable!(),
        }
    }

    // fn run(&mut self, tags: &Vec<String>) {
    fn run(&mut self, tags: &[String]) {
        debug!("running rename::run");
        debug!("  tags: {:?}", tags);
        if tags.len() != 2 || tags.iter().any(|tag| tag.is_empty()) {
            crate::show_error_alfred("Enter 2 tags please!");
        }

        debug!("  calling rename API");
        let r = self
            .pinboard
            .as_ref()
            .unwrap()
            .rename_tag(&tags[0], &tags[1]);
        debug!("  matching result: {:?}", &r);
        match r {
            Err(e) => {
                io::stdout()
                    .write_all(format!("Error: {}", e).as_ref())
                    .expect("Couldn't write to stdout");
                process::exit(1);
            }
            Ok(_) => {
                io::stdout()
                    .write_all(b"Successfully renamed tag.")
                    .expect("Couldn't write to stdout");
                if self.config.as_ref().unwrap().auto_update_cache {
                    self.update_cache();
                }
            }
        }
    }
}
