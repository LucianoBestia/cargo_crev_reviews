// cln_methods_verify_mod.rs

use function_name::named;
use lazy_static::__Deref;
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::auto_generated_mod::common_structs_mod::*;
use crate::auto_generated_mod::srv_methods;

use dev_bestia_html_templating as tmplt;
use dev_bestia_string_utils::*;

// use crate::on_click;
use crate::html_mod::*;
use crate::*;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref VERIFY_ITEM_DATA: Mutex<VerifyItemData> = Mutex::new(VerifyItemData::default());
    static ref VERIFY_LIST_DATA: Mutex<VerifyListData> = Mutex::new(VerifyListData::default());
}

impl tmplt::HtmlTemplatingDataTrait for VerifyItemData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("VerifyItemData")
    }

    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "wt_row_number" => format!("{}.", pos_cursor + 1),
            "wt_status" => self.status.clone(),
            "wt_my_review" => self.my_review.clone(),
            "wt_crate_name" => self.crate_name.clone(),
            "wt_crate_version" => self.crate_version.clone(),
            "wt_crate_name_version" => utils_mod::join_crate_version(&self.crate_name, &self.crate_version),
            "wt_published_by" => self.published_by.clone(),
            "wt_cargo_crev_reviews_version" => env!("CARGO_PKG_VERSION").to_string(),
            "wt_status_class" => format!("review_header0_cell left c_{}", &self.status),
            "wt_my_review_class" => {
                if vec!["strong", "positive", "neutral", "negative"].contains(&self.my_review.as_str()) {
                    format!("review_header0_cell left c_{}", &self.my_review)
                } else {
                    "review_header0_cell left".to_string()
                }
            }
            "wt_published_by_class" => format!("review_header0_cell left c_{}", &self.trusted_publisher),
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }
}

impl tmplt::HtmlTemplatingDataTrait for VerifyListData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("VerifyListData")
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "wt_cargo_crev_reviews_version" => s!(env!("CARGO_PKG_VERSION")),
            "wt_project_dir" => s!(self.project_dir),
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }

    /// renders sub-template: "stmplt_" or "wtmplt_"
    fn process_sub_template(&self, template_name: &str, sub_templates: &Vec<tmplt::utils::SubTemplate>) -> Vec<tmplt::utils::Node> {
        log::info!("{}", template_name);
        match template_name {
            "wtmplt_VerifyItemData" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for (row_number, verify_item) in self.list_of_verify.iter().enumerate() {
                    let vec_node = unwrap!(tmplt::utils::process_template_raw_to_nodes(
                        verify_item,
                        &sub_template.template,
                        tmplt::utils::HtmlOrSvg::Html,
                        "",
                        row_number,
                    ));
                    nodes.extend_from_slice(&vec_node);
                }
                // return
                nodes
            }
            _ => tmplt::utils::match_else_for_process_sub_template(&self.data_model_name(), template_name),
        }
    }
}

#[named]
pub fn request_verify_list(_element_id: &str) {
    log::info!("{}", function_name!());
    let request_data = RpcEmptyData {};
    srv_methods::srv_verify_project(request_data);
}

#[named]
pub fn cln_verify_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    *VERIFY_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let data = VERIFY_LIST_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };

    inject_into_html(&html_after_process);
    navigation_on_click();

    // on_click for every row of the list
    for (row_number, _item) in VERIFY_LIST_DATA.lock().unwrap().list_of_verify.iter().enumerate() {
        row_on_click!("crate_name_version", row_number, open_all_links);
    }
}

#[named]
pub fn request_review_edit_or_new(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_edit_or_new(request_data);
}

#[named]
fn open_all_links(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_number];

    let url = format!("https://web.crev.dev/rust-reviews/crate/{}/", item.crate_name);
    unwrap!(w::window().open_with_url(&url));

    let url = format!("https://lib.rs/crates/{}", item.crate_name);
    unwrap!(w::window().open_with_url(&url));

    let url = format!("https://crates.io/crates/{}/{}", item.crate_name, item.crate_version);
    unwrap!(w::window().open_with_url(&url));

    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_open_source_code(request_data);

    // list versions for this crate
    let url = format!(
        "http://{}:{}/{}/index.html#version_list/{}",
        SERVER_HOST.as_str(),
        SERVER_PORT.as_str(),
        SERVER_FIRST_SUBDIRECTORY.as_str(),
        item.crate_name,
    );
    unwrap!(w::window().open_with_url(&url));
}
