use rfd::FileDialog;

pub fn dialog_builder() -> FileDialog {
    FileDialog::new().add_filter("*", &["ncm", "mp3"])
}