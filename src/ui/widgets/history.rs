use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use self::imp::Page;

mod imp {

    use glib::subclass::InitializingObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib, CompositeTemplate, Label};

    use crate::ui::widgets::item::ItemPage;
    use crate::ui::widgets::movie::MoviePage;

    pub enum Page {
        Movie(Box<gtk::Widget>),
        Item(Box<gtk::Widget>),
    }

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/moe/tsukimi/history.ui")]
    pub struct HistoryPage {
        #[template_child]
        pub historygrid: TemplateChild<gtk::GridView>,
        #[template_child]
        pub spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub historyscrolled: TemplateChild<gtk::ScrolledWindow>,
        pub selection: gtk::SingleSelection,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for HistoryPage {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "HistoryPage";
        type Type = super::HistoryPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for HistoryPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            let spinner = self.spinner.get();
            spinner.set_visible(true);
            let (sender, receiver) = async_channel::bounded::<Vec<crate::ui::network::Resume>>(1);
            crate::ui::network::runtime().spawn(glib::clone!(@strong sender => async move {
                let history_results = crate::ui::network::resume().await.unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    Vec::<crate::ui::network::Resume>::new()
                });
                sender.send(history_results).await.expect("history results not received.");
            }));
            let store = gio::ListStore::new::<glib::BoxedAnyObject>();
            glib::spawn_future_local(glib::clone!(@weak store=> async move {
                while let Ok(history_results) = receiver.recv().await {
                    for result in history_results {
                        let object = glib::BoxedAnyObject::new(result);
                        store.append(&object);
                    }
                    spinner.set_visible(false);
                }
            }));

            self.selection.set_model(Some(&store));
            let factory = gtk::SignalListItemFactory::new();
            factory.connect_bind(move |_factory, item| {
                let listitem = item.downcast_ref::<gtk::ListItem>().unwrap();
                let entry = listitem.item().and_downcast::<glib::BoxedAnyObject>().unwrap();
                let result: std::cell::Ref<crate::ui::network::Resume> = entry.borrow();
                let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
                let imgbox ;
                if result.ParentThumbItemId.is_some() {
                    imgbox = crate::ui::image::set_thumbimage(result.ParentThumbItemId.as_ref().expect("").clone());
                } else {
                    if result.Type == "Movie" {
                        imgbox = crate::ui::image::set_backdropimage(result.Id.clone());
                    } else {
                        imgbox = crate::ui::image::set_backdropimage(result.SeriesId.as_ref().expect("").to_string());
                    }
                }
                imgbox.set_size_request(290, 169);
                vbox.append(&imgbox);
                let label = Label::new(Some(&result.Name));
                let labeltype = Label::new(Some(&result.Type));
                if result.Type == "Episode" {
                    let markup = format!("{}",result.SeriesName.as_ref().expect("").clone());
                    label.set_markup(markup.as_str());
                    let markup = format!("<span color='lightgray' font='10'>S{}E{}: {}</span>", result.ParentIndexNumber.as_ref().expect("").clone(), result.IndexNumber.as_ref().expect("").clone(), result.Name);
                    labeltype.set_markup(markup.as_str());
                } else {
                    let markup = format!("{}", result.Name);
                    label.set_markup(markup.as_str());
                    let markup = format!("<span color='lightgray' font='10'>{}</span>", result.Type);
                    labeltype.set_markup(markup.as_str());
                }
                label.set_wrap(true);
                label.set_size_request(-1, 5);
                label.set_ellipsize(gtk::pango::EllipsizeMode::End);
                labeltype.set_ellipsize(gtk::pango::EllipsizeMode::End);
                label.set_size_request(-1, 5);
                vbox.append(&label);
                vbox.append(&labeltype);
                listitem.set_child(Some(&vbox));
            });
            factory.connect_unbind(|_, item| {
                let listitem = item.downcast_ref::<gtk::ListItem>().unwrap();
                listitem.set_child(None::<&gtk::Widget>);
            });
            self.historygrid.set_factory(Some(&factory));
            self.historygrid.set_model(Some(&self.selection));
            self.historygrid.connect_activate(glib::clone!(@weak obj => move |gridview, position| {
                let model = gridview.model().unwrap();
                let item = model.item(position).and_downcast::<glib::BoxedAnyObject>().unwrap();
                let result: std::cell::Ref<crate::ui::network::Resume> = item.borrow();
                let item_page;
                if result.Type == "Movie" {
                    item_page = Page::Movie(Box::new(MoviePage::new(result.Id.clone(),result.Name.clone()).into()));
                } else {
                    item_page = Page::Item(Box::new(ItemPage::new(result.ParentThumbItemId.as_ref().expect("msg").clone()).into()));
                }
                obj.set(item_page);
            }));
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for HistoryPage {}

    // Trait shared by all windows
    impl WindowImpl for HistoryPage {}

    // Trait shared by all application windows
    impl ApplicationWindowImpl for HistoryPage {}

    impl adw::subclass::navigation_page::NavigationPageImpl for HistoryPage {}
}

glib::wrapper! {
    pub struct HistoryPage(ObjectSubclass<imp::HistoryPage>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget ,adw::NavigationPage,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Default for HistoryPage {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryPage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    fn set(&self, page: Page) {
        let imp = imp::HistoryPage::from_obj(self);
        let widget = match page {
            Page::Movie(widget) => widget,
            Page::Item(widget) => widget,
        };
        imp.historyscrolled.set_child(Some(&*widget));
    }
}
