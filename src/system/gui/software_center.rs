use anyhow::Result;
use gtk4 as gtk;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SoftwareCenter {
    window: gtk::ApplicationWindow,
    package_list: gtk::ListBox,
    search_entry: gtk::SearchEntry,
    package_manager: Rc<RefCell<crate::system::package_manager::PackageManager>>,
}

impl SoftwareCenter {
    pub fn new(application: &gtk::Application) -> Result<Self> {
        let window = gtk::ApplicationWindow::new(application);
        window.set_title(Some("xBitOS Software Center"));
        window.set_default_size(800, 600);

        let header = gtk::HeaderBar::new();
        let search_entry = gtk::SearchEntry::new();
        header.pack_end(&search_entry);
        window.set_titlebar(Some(&header));

        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let package_list = gtk::ListBox::new();
        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_child(Some(&package_list));
        main_box.append(&scrolled);

        window.set_child(Some(&main_box));

        let instance = Self {
            window,
            package_list,
            search_entry,
            package_manager: Rc::new(RefCell::new(
                crate::system::package_manager::PackageManager::new(),
            )),
        };

        instance.setup_signals();
        instance.load_packages()?;

        Ok(instance)
    }

    fn setup_signals(&self) {
        let package_manager = self.package_manager.clone();
        let package_list = self.package_list.clone();

        self.search_entry.connect_search_changed(move |entry| {
            let query = entry.text();
            let packages = package_manager.borrow().search_packages(&query);
            
            // تحديث قائمة الحزم
            package_list.foreach(|child| package_list.remove(child));
            
            for package in packages {
                let row = create_package_row(package);
                package_list.append(&row);
            }
        });
    }

    fn load_packages(&self) -> Result<()> {
        let packages = self.package_manager.borrow().get_all_packages()?;
        
        for package in packages {
            let row = create_package_row(&package);
            self.package_list.append(&row);
        }

        Ok(())
    }

    pub fn show(&self) {
        self.window.show();
    }
}

fn create_package_row(package: &crate::system::software::SoftwarePackage) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::new();
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    
    let name_label = gtk::Label::new(Some(&package.name));
    let desc_label = gtk::Label::new(Some(&package.description));
    let install_button = if package.installed {
        gtk::Button::with_label("Remove")
    } else {
        gtk::Button::with_label("Install")
    };

    hbox.append(&name_label);
    hbox.append(&desc_label);
    hbox.append(&install_button);
    
    row.set_child(Some(&hbox));
    row
} 