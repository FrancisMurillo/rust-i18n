#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("examples/app/locales");

fn main() {}

#[test]
fn test_example_app() {
    rust_i18n::set_locale("en");
    assert_eq!(t!("hello", name = "Longbridge"), "Hello, Longbridge!");
    assert_eq!(t!("view.buttons.ok"), "Ok");
    assert_eq!(t!("view.buttons.cancel"), "Cancel");
    assert_eq!(
        t!("view.datetime.about_x_hours", count = "10"),
        "about 10 hours"
    );

    assert_eq!(
        t!("hello", locale = "fr", name = "Longbridge"),
        "Bonjour, Longbridge!"
    );
    rust_i18n::set_locale("fr");
    assert_eq!(t!("hello", name = "Longbridge"), "Bonjour, Longbridge!");
    assert_eq!(
        t!("view.datetime.about_x_hours", count = "10"),
        "environ 10 heures"
    );
}