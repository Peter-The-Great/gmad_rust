use bootil::bstring::BString;
use bootil::file::read;
use bootil::data::json::import;
use bootil::data::tree::Tree;
use bootil::string::get_lower;
use bootil::string::test::wildcard;

pub struct CAddonJson {
    pub m_strError: BString,
    pub m_Title: BString,
    pub m_Description: BString,
    pub m_AddonType: BString,
    pub m_Ignores: Vec<BString>,
    pub m_Tags: Vec<BString>,
}

impl CAddonJson {
    pub fn new(strInfoFile: BString) -> CAddonJson {
        let mut addon_json = CAddonJson {
            m_strError: BString::new(),
            m_Title: BString::new(),
            m_Description: BString::new(),
            m_AddonType: BString::new(),
            m_Ignores: Vec::new(),
            m_Tags: Vec::new(),
        };

        let mut strFileContents = BString::new();

        if !read(&strInfoFile, &mut strFileContents) {
            addon_json.m_strError = "Couldn't find file".into();
            return addon_json;
        }

        let mut tree = Tree::new();

        if !import(&mut tree, strFileContents.as_str()) {
            addon_json.m_strError = "Couldn't parse json".into();
            return addon_json;
        }

        addon_json.m_Title = tree.child_value("title");

        if addon_json.m_Title.is_empty() {
            addon_json.m_strError = "title is empty!".into();
            return addon_json;
        }

        addon_json.m_Description = tree.child_value("description", "Description");

        addon_json.m_AddonType = tree.child_value("type", "").to_lowercase();

        if addon_json.m_AddonType.is_empty() {
            addon_json.m_strError = "type is empty!".into();
            return addon_json;
        }

        if !Addon::Tags::type_exists(&addon_json.m_AddonType) {
            addon_json.m_strError = "type isn't a supported type!".into();
            return addon_json;
        }

        if let Some(tags) = tree.get_child("tags") {
            if tags.children().len() > 2 {
                addon_json.m_strError = "too many tags - specify 2 only!".into();
                return addon_json;
            }

            for child in tags.children() {
                let tag = child.value();
                if !tag.is_empty() {
                    addon_json.m_Tags.push(tag);
                    addon_json.m_Tags
                        .last_mut()
                        .map(|t| *t = get_lower(t));
                    if !Addon::Tags::tag_exists(&addon_json.m_Tags.last().unwrap()) {
                        addon_json.m_strError = "tag isn't a supported word!".into();
                        return addon_json;
                    }
                }
            }
        }

        if let Some(ignores) = tree.get_child("ignore") {
            for child in ignores.children() {
                addon_json.m_Ignores.push(child.value());
            }
        }

        addon_json
    }

    pub fn remove_ignored_files(&self, files: &mut Vec<BString>) {
        let mut new_files: Vec<BString> = Vec::new();
        for f in files {
            let mut b_skip_file = false;

            if f == "addon.json" {
                continue;
            }

            let str_low = get_lower(f);
            if wildcard("*thumbs.db", &str_low) {
                continue;
            }
            if wildcard("*desktop.ini", &str_low) {
                continue;
            }

            if f == ".DS_Store" {
                continue;
            }
            if wildcard("*/.DS_Store", f) {
                continue;
            }

            for ignore in &self.m_Ignores {
                if wildcard(ignore, f) {
                    b_skip_file = true;
                    break;
                }
            }

            if !b_skip_file {
                new_files.push(f.clone());
            }
        }

        *files = new_files;
    }

    pub fn build_description(&self) -> BString {
        let mut tree = Tree::new();
        tree.set_child("description", self.GetDescription());
        tree.set_child("type", self.GetType());
        let tags = tree.get_or_add_child("tags");
        for tag in &self.m_Tags {
            tags.add_child().value(tag);
        }
        let mut str_output = BString::new();
        tree.export(&mut str_output, true);
        str_output
    }

    pub fn GetError(&self) -> &BString {
        &self.m_strError
    }

    pub fn GetTitle(&self) -> &BString {
        &self.m_Title
    }

    pub fn GetDescription(&self) -> &BString {
        &self.m_Description
    }

    pub fn GetType(&self) -> &BString {
        &self.m_AddonType
    }
}