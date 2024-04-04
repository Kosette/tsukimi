use super::mpv;
use super::network;
use super::network::Back;
use super::network::SeriesInfo;
use gtk::glib;
use gtk::prelude::*;
use gtk::Orientation;

pub fn newmediadropsel(playbackinfo: network::Media, info: SeriesInfo) -> gtk::Box {
    let hbox = gtk::Box::new(Orientation::Horizontal, 5);
    hbox.set_valign(gtk::Align::End);
    hbox.set_vexpand(true);
    let leftvbox = gtk::Box::new(Orientation::Vertical, 5);
    leftvbox.set_margin_start(80);
    leftvbox.set_margin_top(80);
    leftvbox.set_margin_bottom(20);
    leftvbox.set_halign(gtk::Align::Start);
    leftvbox.set_valign(gtk::Align::End);
    let markup = format!(
        "<b>{}\n\nSeason {} : Episode {}</b>",
        info.Name, info.ParentIndexNumber, info.IndexNumber
    );
    let label = gtk::Label::new(Some(&info.Name));
    label.set_markup(markup.as_str());
    label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    leftvbox.append(&label);
    hbox.append(&leftvbox);
    let vbox = gtk::Box::new(Orientation::Vertical, 5);
    vbox.set_margin_start(20);
    vbox.set_margin_end(80);
    vbox.set_margin_bottom(20);
    vbox.set_halign(gtk::Align::End);
    vbox.set_valign(gtk::Align::End);
    vbox.set_hexpand(true);
    let namelist = gtk::StringList::new(&[]);

    let sublist = gtk::StringList::new(&[]);

    let mut _set = 1;
    for media in playbackinfo.MediaSources.clone() {
        namelist.append(&media.Name);
        if _set == 1 {
            for stream in media.MediaStreams {
                if stream.Type == "Subtitle" {
                    if let Some(d) = stream.DisplayTitle {
                        sublist.append(&d);
                    } else {
                        println!("No value");
                    }
                }
            }
        }
        _set = 0;
    }

    let namedropdown = gtk::DropDown::new(Some(namelist), Option::<gtk::Expression>::None);
    let subdropdown = gtk::DropDown::new(Some(sublist.clone()), Option::<gtk::Expression>::None);
    let playback_info = playbackinfo.clone();

    namedropdown.connect_selected_item_notify(move |dropdown| {
        let selected = dropdown.selected_item();
        let selected = selected.and_downcast_ref::<gtk::StringObject>().unwrap();
        let selected = selected.string();
        for _i in 0..sublist.n_items() {
            sublist.remove(0);
        }
        for media in playbackinfo.MediaSources.clone() {
            if media.Name == selected {
                for stream in media.MediaStreams {
                    if stream.Type == "Subtitle" {
                        if let Some(d) = stream.DisplayTitle {
                            sublist.append(&d);
                        } else {
                            println!("No value");
                        }
                    }
                }
                break;
            }
        }
    });
    vbox.append(&namedropdown);
    vbox.append(&subdropdown);
    let info = info.clone();
    let playbutton = gtk::Button::with_label("Play");
    playbutton.connect_clicked(move |button| {
        button.set_label("Playing...");
        button.set_sensitive(false);
        let nameselected = namedropdown.selected_item();
        let nameselected = nameselected
            .and_downcast_ref::<gtk::StringObject>()
            .unwrap();
        let nameselected = nameselected.string();
        let subselected = subdropdown.selected_item();
        if subselected.is_none() {
            for media in playback_info.MediaSources.clone() {
                if media.Name == nameselected {
                    let directurl = media.DirectStreamUrl.clone();
                    let back = Back {
                        id: info.Id.clone(),
                        mediasourceid: media.Id.clone(),
                        playsessionid: playback_info.PlaySessionId.clone(),
                        tick: 0.,
                    };
                    play_event(button.clone(), directurl, media.Name, back);
                    return;
                }
            }
            return;
        }
        let subselected = subselected.and_downcast_ref::<gtk::StringObject>().unwrap();
        let subselected = subselected.string();
        for media in playback_info.MediaSources.clone() {
            if media.Name == nameselected.to_string() {
                for mediastream in media.MediaStreams {
                    // let sub = subselected.clone();
                    if mediastream.Type == "Subtitle" {
                        let displaytitle = mediastream.DisplayTitle.unwrap_or("".to_string());
                        if displaytitle == subselected {
                            if let Some(directurl) = media.DirectStreamUrl.clone() {
                                if mediastream.IsExternal == true {
                                    if let Some(_suburl) = mediastream.DeliveryUrl.clone() {
                                        let back = Back {
                                            id: info.Id.clone(),
                                            mediasourceid: media.Id.clone(),
                                            playsessionid: playback_info.PlaySessionId.clone(),
                                            tick: 0.,
                                        };
                                        play_event(
                                            button.clone(),
                                            Some(directurl),
                                            media.Name,
                                            back,
                                        );
                                        return;
                                    }
                                } else {
                                    let back = Back {
                                        id: info.Id.clone(),
                                        mediasourceid: media.Id.clone(),
                                        playsessionid: playback_info.PlaySessionId.clone(),
                                        tick: 0.,
                                    };
                                    play_event(button.clone(), Some(directurl), media.Name, back);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    vbox.append(&playbutton);
    hbox.append(&vbox);
    hbox
}

pub fn play_event(button: gtk::Button, directurl: Option<String>, name: String, back: Back) {
    let (sender, receiver) = async_channel::bounded(1);
    gtk::gio::spawn_blocking(move || {
        sender
            .send_blocking(false)
            .expect("The channel needs to be open.");
        match mpv::event::play(directurl.expect("no url"), None, Some(name), back) {
            Ok(_) => {
                sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
            }
            Err(e) => {
                eprintln!("Failed to play: {}", e);
            }
        };
    });
    glib::spawn_future_local(glib::clone!(@weak button =>async move {
        while let Ok(enable_button) = receiver.recv().await {
            if enable_button {
                button.set_label("Play");
            }
            button.set_sensitive(enable_button);
        }
    }));
}
