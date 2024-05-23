use zed_extension_api as zed;

struct SQLExtension;

impl zed::Extension for SQLExtension {
    fn new() -> Self {
        todo!()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        todo!()
    }
}

zed::register_extension!(SQLExtension);
