use std::rc::Rc;

use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

pub(crate) fn build_application(current_account: String, accounts: Vec<String>) -> anyhow::Result<()> {
    let application = MainWindow::new()?;

    let the_model: Rc<VecModel<SharedString>> = Rc::new(VecModel::from(
        accounts
            .iter()
            .map(|s| s.into())
            .collect::<Vec<SharedString>>(),
    ));

    application.set_current_account(current_account.into());

    application.set_accounts(ModelRc::from(the_model.clone()));

    application.on_click_ok(|x| {
        println!("{}", x);
    });

    application.run()?;
    Ok(())
}