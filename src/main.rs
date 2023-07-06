mod ui;

fn main() -> anyhow::Result<()> {
    ui::build_application(
        "current_account".into(),
        vec!["114514".into(), "1919810".into()],
    )?;
    Ok(())
}
