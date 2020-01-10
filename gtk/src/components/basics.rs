use gtk::prelude::*;

use crate::components::validated_text_entry::ValidatedTextEntry;
use crate::conversions::{
    parse_distance, parse_duration, parse_hours_minutes, render_distance, render_duration,
    render_hours_minutes,
};

pub fn date_c(date: &chrono::Date<chrono_tz::Tz>) -> gtk::Label {
    gtk::Label::new(Some(&format!("{}", date.format("%B %e, %Y"))))
}

pub fn time_c(time: &chrono::NaiveTime) -> gtk::Label {
    gtk::Label::new(Some(&format!("{}", time.format("%H:%M"))))
}

pub fn time_edit_c(
    time: &chrono::NaiveTime,
    on_update: Box<dyn Fn(chrono::NaiveTime)>,
) -> ValidatedTextEntry<chrono::NaiveTime> {
    ValidatedTextEntry::new(
        time.clone(),
        Box::new(|s| render_hours_minutes(s)),
        Box::new(|s| parse_hours_minutes(s)),
        on_update,
    )
}

pub fn distance_c(distance: &dimensioned::si::Meter<f64>) -> gtk::Label {
    gtk::Label::new(Some(&format!("{} km", render_distance(distance))))
}

pub fn distance_edit_c(
    distance: &Option<dimensioned::si::Meter<f64>>,
    on_update: Box<dyn Fn(Option<dimensioned::si::Meter<f64>>)>,
) -> ValidatedTextEntry<Option<dimensioned::si::Meter<f64>>> {
    ValidatedTextEntry::new(
        distance.clone(),
        Box::new(|s| s.map(|s_| render_distance(&s_)).unwrap_or(String::from(""))),
        Box::new(|s| parse_distance(s)),
        on_update,
    )
}

pub fn duration_c(duration: &dimensioned::si::Second<f64>) -> gtk::Label {
    gtk::Label::new(Some(&render_duration(duration)))
}

pub fn duration_edit_c(
    duration: &Option<dimensioned::si::Second<f64>>,
    on_update: Box<dyn Fn(Option<dimensioned::si::Second<f64>>)>,
) -> ValidatedTextEntry<Option<dimensioned::si::Second<f64>>> {
    ValidatedTextEntry::new(
        duration.clone(),
        Box::new(|s| s.map(|s_| render_duration(&s_)).unwrap_or(String::from(""))),
        Box::new(|s| parse_duration(s)),
        on_update,
    )
}

pub fn setting_with_label_c<A: IsA<gtk::Widget>>(label: &str, selector: A) -> gtk::Box {
    let widget = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    widget.pack_start(&gtk::Label::new(Some(label)), false, false, 5);
    widget.pack_start(&selector, false, false, 5);

    widget
}

pub fn entry_setting_c(label: &str, current: &str, on_changed: Box<dyn Fn(&str)>) -> gtk::Box {
    let entry = gtk::Entry::new();
    entry.set_text(current);
    entry.connect_changed(move |v| match v.get_text() {
        Some(ref s) => on_changed(s),
        None => (),
    });

    setting_with_label_c(label, entry)
}

pub fn pulldown_setting_c(
    label: &str,
    options: Vec<(&str, &str)>,
    current: &str,
    on_changed: Box<dyn Fn(&str)>,
) -> gtk::Box {
    let combo = gtk::ComboBoxText::new();
    for (id, option) in options.iter() {
        combo.append(Some(id), option);
    }
    combo.set_active_id(Some(current));
    combo.connect_changed(move |s| match s.get_active_id() {
        Some(val) => on_changed(val.as_str()),
        None => (),
    });

    setting_with_label_c(label, combo)
}
