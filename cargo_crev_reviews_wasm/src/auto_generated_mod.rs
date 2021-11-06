// auto_generated_mod.rs

// generated by `cargo auto` automation task
// Please, don't modify manually.

use crate::cln_methods_review_mod::*;
use crate::cln_methods_verify_mod::*;
use crate::cln_methods_version_mod::*;

/// match the string and call a function
pub async fn match_response_method_and_call_function(response: common_structs_mod::RpcResponse) {
    match response.response_method.as_str() {
        // generator match_response_method start
        "cln_review_edit" => cln_review_edit(response),
        "cln_review_error" => cln_review_error(response),
        "cln_review_list" => cln_review_list(response),
        "cln_review_new" => cln_review_new(response),
        "cln_review_publish_modal" => cln_review_publish_modal(response),
        "cln_verify_list" => cln_verify_list(response),
        "cln_version_list" => cln_version_list(response),
        // generator match_response_method end
        _ => log::error!("Error: Unrecognized response_method {}", response.response_method),
    }
}

// proxy for public functions on server
pub mod srv_methods {
    use crate::html_mod::post_request_await_run_response_method;
    use function_name::named;

    // generator srv_methods start

    #[named]
    pub fn srv_review_delete<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_review_edit<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_review_edit_or_new<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_review_new<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_review_new_version<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_review_open_source_code<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_review_publish<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_review_save<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_reviews_list<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_update_registry_index<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_verify_project<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }

    #[named]
    pub fn srv_version_list<T>(request_data: T)
    where
        T: serde::Serialize,
    {
        let request_method = function_name!();
        post_request_await_run_response_method(request_method, request_data);
    }
    // generator srv_methods end
}

pub mod common_structs_mod {
    // generator common_structs_mod start
    // common_structs_mod.rs

    //! common structs between the backend and frontend
    //! One automation task will copy it over to cargo_crev_reviews_wasm/auto_generated_mod.rs !

    use serde::{Deserialize, Serialize};

    // region: platform wide structs

    /// the request_method will be processed on the server
    #[derive(Serialize, Deserialize, Debug)]
    pub struct RpcRequest {
        pub request_method: String,
        pub request_data: serde_json::Value,
    }

    /// the response_method will be processed on the client
    #[derive(Serialize, Deserialize, Debug)]
    pub struct RpcResponse {
        pub response_method: String,
        pub response_data: serde_json::Value,
        pub response_html: String,
    }

    /// generic message for Rpc
    #[derive(Serialize, Deserialize, Debug)]
    pub struct RpcMessageData {
        pub message: String,
    }

    /// generic empty data for Rpc
    #[derive(Serialize, Deserialize, Debug)]
    pub struct RpcEmptyData {}
    // endregion: platform wide structs

    // region: review

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct ReviewFilterData {
        pub crate_name: String,
        pub crate_version: Option<String>,
        pub old_crate_version: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct ReviewItemData {
        pub crate_name: String,
        pub crate_version: String,
        pub date: String,
        pub thoroughness: String,
        pub understanding: String,
        pub rating: String,
        pub comment_md: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct ReviewListData {
        pub filter: String,
        pub list_of_review: Vec<ReviewItemData>,
    }

    // endregion: review
    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct VerifyItemData {
        /// status: none, pass, warn, yanked
        pub status: String,
        /// rating if exists, version number if exists for crate
        pub my_review: String,
        pub crate_name: String,
        pub crate_version: String,
        pub published_by: String,
        pub trusted_publisher: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct VerifyListData {
        pub project_dir: String,
        pub list_of_verify: Vec<VerifyItemData>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct VersionItemData {
        pub crate_name: String,
        pub crate_version: String,
        pub yanked: bool,
        pub published_by_login: Option<String>,
        pub published_date: String,
        pub is_src_cached: Option<bool>,
        pub my_review: Option<ReviewItemData>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct VersionListData {
        pub list_of_version: Vec<VersionItemData>,
    }
    // generator common_structs_mod end
}
