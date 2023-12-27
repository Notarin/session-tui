use cursive::{
    align::HAlign,
    event::EventResult,
    view::Scrollable,
    views::{CircularFocus, Dialog, OnEventView, SelectView, TextView},
    Cursive, With,
};

use crate::{
    check_if_new, connect_to_session, create_new_session, multiplexers::Session, SESSIONS,
};

pub(crate) fn display_pick_list() {
    let mut tui: cursive::CursiveRunnable = cursive::default();

    let mut options: Vec<String> = SESSIONS
        .iter()
        .map(|session| session.name.clone())
        .collect();

    for option in &mut options {
        *option = option.replace('\t', " ");
        while option.contains("  ") {
            *option = option.replace("  ", " ");
        }
    }

    if options.is_empty() {
        create_new_session(&mut tui);
        return;
    }

    options.push(String::from("New session"));
    let mut menu: SelectView = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    menu.add_all_str(options);

    menu.set_on_submit(check_if_new);

    // We add vim movement keys
    let menu: OnEventView<SelectView> = OnEventView::new(menu)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    tui.load_toml(include_str!("../style.toml")).unwrap();
    tui.add_layer(Dialog::around(menu.scrollable()).title("Select a session."));
    // .fixed_size((40, 15))

    tui.run();
}

pub(crate) fn display_warning(tui: &mut Cursive, session: Session) {
    let view = TextView::new(
        "Warning! This session is already attached. Continuing will detach this session from its client!",
    );
    let dialog = Dialog::around(view)
        .title("Warning!")
        .button("Okay", move |tui| connect_to_session(tui, &session))
        .wrap_with(CircularFocus::new)
        .wrap_tab();
    tui.add_layer(dialog);
}
