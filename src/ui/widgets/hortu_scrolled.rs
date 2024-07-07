use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib, template_callbacks, CompositeTemplate};

use crate::client::structs::SimpleListItem;
use crate::ui::provider::tu_item::TuItem;
use crate::ui::widgets::fix::ScrolledWindowFixExt;
use crate::utils::spawn;

use super::tu_list_item::TuListItem;

mod imp {
    use std::cell::OnceCell;

    use glib::subclass::InitializingObject;
    use gtk::gio;

    use crate::{client::structs::SimpleListItem, ui::widgets::window::Window};

    use super::*;

    #[derive(Debug, Default, CompositeTemplate, glib::Properties)]
    #[template(resource = "/moe/tsukimi/hortu_scrolled.ui")]
    #[properties(wrapper_type = super::HortuScrolled)]
    pub struct HortuScrolled {
        #[property(get, set, construct_only)]
        pub isresume: OnceCell<bool>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child]
        pub scrolled: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub list: TemplateChild<gtk::ListView>,
        #[template_child]
        pub revealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub morebutton: TemplateChild<gtk::Button>,

        pub selection: gtk::SingleSelection,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HortuScrolled {
        const NAME: &'static str = "HortuScrolled";
        type Type = super::HortuScrolled;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for HortuScrolled {
        fn constructed(&self) {
            self.parent_constructed();
            self.scrolled.fix();

            let store = gio::ListStore::new::<glib::BoxedAnyObject>();

            self.selection.set_model(Some(&store));

            self.list.set_model(Some(&self.selection));

            self.list
                .set_factory(Some(&self.factory(*self.isresume.get().unwrap_or(&false))));

            self.list.connect_activate(
                glib::clone!(@weak self as imp => move |listview, position| {
                    let window = imp.obj().root().and_downcast::<Window>().unwrap();
                    let model = listview.model().unwrap();
                    let item = model.item(position).and_downcast::<glib::BoxedAnyObject>().unwrap();
                    let result: std::cell::Ref<SimpleListItem> = item.borrow();
                    imp.activate(window, &result);
                }),
            );
        }
    }

    impl WidgetImpl for HortuScrolled {}

    impl BinImpl for HortuScrolled {}

    impl HortuScrolled {
        fn factory(&self, is_resume: bool) -> gtk::SignalListItemFactory {
            let factory = gtk::SignalListItemFactory::new();
            factory.connect_bind(move |_, item| {
                let list_item = item
                    .downcast_ref::<gtk::ListItem>()
                    .expect("Needs to be ListItem");
                let entry = item
                    .downcast_ref::<gtk::ListItem>()
                    .expect("Needs to be ListItem")
                    .item()
                    .and_downcast::<glib::BoxedAnyObject>()
                    .expect("Needs to be BoxedAnyObject");
                let item: std::cell::Ref<SimpleListItem> = entry.borrow();
                if list_item.child().is_none() {
                    let tu_item = TuItem::from_simple(&item, None);
                    let list_child = TuListItem::new(tu_item, &item.latest_type, is_resume);
                    list_item.set_child(Some(&list_child));
                }
            });
            factory
        }

        fn activate(&self, window: crate::ui::widgets::window::Window, result: &SimpleListItem) {
            let (view, title_var) = match window.current_view_name().as_str() {
                "homepage" => (&window.imp().homeview, "HOME_TITLE"),
                "searchpage" => (&window.imp().searchview, "SEARCH_TITLE"),
                "historypage" => (&window.imp().historyview, "HISTORY_TITLE"),
                _ => (&window.imp().searchview, "SEARCH_TITLE"),
            };
            window.set_title(&result.name);
            std::env::set_var(title_var, &result.name);

            match result.latest_type.as_str() {
                "Movie" | "Video" => Self::push_page(
                    view,
                    &window,
                    &result.name,
                    crate::ui::widgets::item::ItemPage::new(
                        result.id.clone(),
                        result.id.clone(),
                        result.name.clone(),
                    ),
                ),
                "Series" => Self::push_page(
                    view,
                    &window,
                    &result.name,
                    crate::ui::widgets::item::ItemPage::new(
                        result.id.clone(),
                        result.id.clone(),
                        result.name.clone(),
                    ),
                ),
                "Episode" => Self::push_page(
                    view,
                    &window,
                    &result.series_name.clone().unwrap_or_default(),
                    crate::ui::widgets::item::ItemPage::new(
                        result.series_id.as_ref().unwrap().clone(),
                        result.id.clone(),
                        result.series_name.clone().unwrap_or("".to_string()),
                    ),
                ),
                "Actor" | "Person" | "Director" => Self::push_page(
                    view,
                    &window,
                    &result.name,
                    crate::ui::widgets::actor::ActorPage::new(&result.id),
                ),
                "BoxSet" => Self::push_page(
                    view,
                    &window,
                    &result.name,
                    crate::ui::widgets::boxset::BoxSetPage::new(&result.id),
                ),
                "MusicAlbum" => {
                    let item = TuItem::from_simple(result, None);
                    Self::push_page(
                        view,
                        &window,
                        &result.name,
                        crate::ui::widgets::music_album::AlbumPage::new(item),
                    )
                }
                "CollectionFolder" => {
                    let item = TuItem::from_simple(result, None);
                    Self::push_page(
                        view,
                        &window,
                        &result.name,
                        crate::ui::widgets::list::ListPage::new(
                            item.id(),
                            item.collection_type().unwrap_or_default(),
                        ),
                    )
                }
                _ => {}
            }
        }

        fn push_page<T: 'static + Clone + gtk::prelude::IsA<adw::NavigationPage>>(
            view: &adw::NavigationView,
            window: &crate::ui::widgets::window::Window,
            tag: &str,
            page: T,
        ) {
            if view.find_page(tag).is_some() {
                view.pop_to_tag(tag);
            } else {
                let item_page = page;
                item_page.set_tag(Some(tag));
                view.push(&item_page);
                window.set_pop_visibility(true);
            }
        }
    }
}

glib::wrapper! {
    /// A scrolled list of items.
    pub struct HortuScrolled(ObjectSubclass<imp::HortuScrolled>)
        @extends gtk::Widget, adw::Bin, @implements gtk::Accessible;
}

#[template_callbacks]
impl HortuScrolled {
    pub fn new(is_resume: bool) -> Self {
        glib::Object::builder()
            .property("isresume", is_resume)
            .build()
    }

    pub fn set_morebutton(&self) {
        let imp = self.imp();
        imp.morebutton.set_visible(true);
    }

    pub fn set_items(&self, items: &Vec<SimpleListItem>) {
        if items.is_empty() {
            return;
        }

        self.set_visible(true);

        let items = items.to_owned();
        let items = items.clone();

        let imp = self.imp();
        let store = imp
            .selection
            .model()
            .unwrap()
            .downcast::<gio::ListStore>()
            .unwrap();

        imp.revealer.set_reveal_child(true);

        spawn(glib::clone!(@weak store=> async move {
            for result in items {
                let object = glib::BoxedAnyObject::new(result);
                store.append(&object);
                gtk::glib::timeout_future(std::time::Duration::from_millis(30)).await;
            }

        }));
    }

    pub fn set_title(&self, title: &str) {
        self.imp().label.set_text(title);
    }

    #[template_callback]
    fn on_rightbutton_clicked(&self) {
        self.anime(true);
    }

    #[template_callback]
    fn on_leftbutton_clicked(&self) {
        self.anime(false);
    }

    fn anime(&self, is_right: bool) {
        let scrolled = self.imp().scrolled.get();
        let adj = scrolled.hadjustment();

        let Some(clock) = scrolled.frame_clock() else {
            return;
        };

        let start = adj.value();
        let end = if is_right {
            start + 400.0
        } else {
            start - 400.0
        };
        let start_time = clock.frame_time();
        let end_time = start_time + 1000 * 400;
        scrolled.add_tick_callback(move |_view, clock| {
            let now = clock.frame_time();
            if now < end_time && adj.value() != end {
                let mut t = (now - start_time) as f64 / (end_time - start_time) as f64;
                t = Self::ease_out_cubic(t);
                adj.set_value(start + t * (end - start));
                glib::ControlFlow::Continue
            } else {
                adj.set_value(end);
                glib::ControlFlow::Break
            }
        });
    }

    fn ease_out_cubic(t: f64) -> f64 {
        let t = t - 1.0;
        t * t * t + 1.0
    }
}
