// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

mod authentication_request;
pub use self::authentication_request::AuthenticationRequest;

mod automation_session;
pub use self::automation_session::AutomationSession;

mod back_forward_list;
pub use self::back_forward_list::BackForwardList;

mod back_forward_list_item;
pub use self::back_forward_list_item::BackForwardListItem;

mod color_chooser_request;
pub use self::color_chooser_request::ColorChooserRequest;

mod context_menu;
pub use self::context_menu::ContextMenu;

mod context_menu_item;
pub use self::context_menu_item::ContextMenuItem;

mod cookie_manager;
pub use self::cookie_manager::CookieManager;

mod device_info_permission_request;
pub use self::device_info_permission_request::DeviceInfoPermissionRequest;

mod download;
pub use self::download::Download;

mod editor_state;
pub use self::editor_state::EditorState;

mod favicon_database;
pub use self::favicon_database::FaviconDatabase;

mod file_chooser_request;
pub use self::file_chooser_request::FileChooserRequest;

mod find_controller;
pub use self::find_controller::FindController;

mod form_submission_request;
pub use self::form_submission_request::FormSubmissionRequest;

mod geolocation_manager;
pub use self::geolocation_manager::GeolocationManager;

mod geolocation_permission_request;
pub use self::geolocation_permission_request::GeolocationPermissionRequest;

mod hit_test_result;
pub use self::hit_test_result::HitTestResult;

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
mod input_method_context;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::input_method_context::InputMethodContext;

mod install_missing_media_plugins_permission_request;
pub use self::install_missing_media_plugins_permission_request::InstallMissingMediaPluginsPermissionRequest;

mod media_key_system_permission_request;
pub use self::media_key_system_permission_request::MediaKeySystemPermissionRequest;

mod navigation_policy_decision;
pub use self::navigation_policy_decision::NavigationPolicyDecision;

mod notification;
pub use self::notification::Notification;

mod notification_permission_request;
pub use self::notification_permission_request::NotificationPermissionRequest;

mod option_menu;
pub use self::option_menu::OptionMenu;

mod permission_request;
pub use self::permission_request::PermissionRequest;

mod plugin;
pub use self::plugin::Plugin;

mod pointer_lock_permission_request;
pub use self::pointer_lock_permission_request::PointerLockPermissionRequest;

mod policy_decision;
pub use self::policy_decision::PolicyDecision;

mod print_custom_widget;
pub use self::print_custom_widget::PrintCustomWidget;

mod print_operation;
pub use self::print_operation::PrintOperation;

mod response_policy_decision;
pub use self::response_policy_decision::ResponsePolicyDecision;

mod security_manager;
pub use self::security_manager::SecurityManager;

mod settings;
pub use self::settings::Settings;

mod uri_request;
pub use self::uri_request::URIRequest;

mod uri_response;
pub use self::uri_response::URIResponse;

mod uri_scheme_request;
pub use self::uri_scheme_request::URISchemeRequest;

#[cfg(any(feature = "v2_36", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
mod uri_scheme_response;
#[cfg(any(feature = "v2_36", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
pub use self::uri_scheme_response::URISchemeResponse;

mod user_content_manager;
pub use self::user_content_manager::UserContentManager;

mod user_media_permission_request;
pub use self::user_media_permission_request::UserMediaPermissionRequest;

mod user_message;
pub use self::user_message::UserMessage;

mod web_context;
pub use self::web_context::WebContext;

mod web_inspector;
pub use self::web_inspector::WebInspector;

mod web_resource;
pub use self::web_resource::WebResource;

mod web_view;
pub use self::web_view::WebView;

mod web_view_base;
pub use self::web_view_base::WebViewBase;

mod website_data_access_permission_request;
pub use self::website_data_access_permission_request::WebsiteDataAccessPermissionRequest;

mod website_data_manager;
pub use self::website_data_manager::WebsiteDataManager;

mod website_policies;
pub use self::website_policies::WebsitePolicies;

mod window_properties;
pub use self::window_properties::WindowProperties;

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
mod application_info;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
pub use self::application_info::ApplicationInfo;

#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
mod geolocation_position;
#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
pub use self::geolocation_position::GeolocationPosition;

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
mod itp_first_party;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::itp_first_party::ITPFirstParty;

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
mod itp_third_party;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::itp_third_party::ITPThirdParty;

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
mod input_method_underline;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::input_method_underline::InputMethodUnderline;

#[cfg(any(feature = "v2_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
mod memory_pressure_settings;
#[cfg(any(feature = "v2_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
pub use self::memory_pressure_settings::MemoryPressureSettings;

mod mime_info;
pub use self::mime_info::MimeInfo;

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
mod navigation_action;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::navigation_action::NavigationAction;

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod network_proxy_settings;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::network_proxy_settings::NetworkProxySettings;

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
mod option_menu_item;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
pub use self::option_menu_item::OptionMenuItem;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
mod script_dialog;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
pub use self::script_dialog::ScriptDialog;

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod security_origin;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::security_origin::SecurityOrigin;

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
mod user_script;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::user_script::UserScript;

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
mod user_style_sheet;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::user_style_sheet::UserStyleSheet;

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
mod web_view_session_state;
#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
pub use self::web_view_session_state::WebViewSessionState;

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod website_data;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::website_data::WebsiteData;

mod enums;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
pub use self::enums::AuthenticationScheme;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::enums::AutomationBrowsingContextPresentation;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::enums::AutoplayPolicy;
pub use self::enums::CacheModel;
pub use self::enums::ContextMenuAction;
pub use self::enums::CookieAcceptPolicy;
pub use self::enums::CookiePersistentStorage;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
pub use self::enums::CredentialPersistence;
pub use self::enums::DownloadError;
pub use self::enums::FaviconDatabaseError;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::enums::HardwareAccelerationPolicy;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::enums::InputPurpose;
pub use self::enums::InsecureContentEvent;
pub use self::enums::JavascriptError;
pub use self::enums::LoadEvent;
#[cfg(any(feature = "v2_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
pub use self::enums::MediaCaptureState;
pub use self::enums::NavigationType;
pub use self::enums::NetworkError;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::enums::NetworkProxyMode;
pub use self::enums::PluginError;
pub use self::enums::PolicyDecisionType;
pub use self::enums::PolicyError;
pub use self::enums::PrintError;
pub use self::enums::PrintOperationResponse;
#[cfg(any(feature = "v2_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
pub use self::enums::ProcessModel;
pub use self::enums::SaveMode;
pub use self::enums::ScriptDialogType;
pub use self::enums::SnapshotError;
pub use self::enums::SnapshotRegion;
pub use self::enums::TLSErrorsPolicy;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
pub use self::enums::UserContentFilterError;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::enums::UserContentInjectedFrames;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::enums::UserMessageError;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::enums::UserScriptInjectionTime;
#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
pub use self::enums::UserStyleLevel;
#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
pub use self::enums::WebProcessTerminationReason;

mod flags;
#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
pub use self::flags::EditorTypingAttributes;
pub use self::flags::FindOptions;
pub use self::flags::HitTestResultContext;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
pub use self::flags::InputHints;
pub use self::flags::SnapshotOptions;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use self::flags::WebsiteDataTypes;

#[doc(hidden)]
pub mod traits {
  pub use super::authentication_request::AuthenticationRequestExt;
  pub use super::automation_session::AutomationSessionExt;
  pub use super::back_forward_list::BackForwardListExt;
  pub use super::back_forward_list_item::BackForwardListItemExt;
  pub use super::color_chooser_request::ColorChooserRequestExt;
  pub use super::context_menu::ContextMenuExt;
  pub use super::context_menu_item::ContextMenuItemExt;
  pub use super::cookie_manager::CookieManagerExt;
  pub use super::download::DownloadExt;
  pub use super::editor_state::EditorStateExt;
  pub use super::favicon_database::FaviconDatabaseExt;
  pub use super::file_chooser_request::FileChooserRequestExt;
  pub use super::find_controller::FindControllerExt;
  pub use super::form_submission_request::FormSubmissionRequestExt;
  pub use super::geolocation_manager::GeolocationManagerExt;
  pub use super::hit_test_result::HitTestResultExt;
  #[cfg(any(feature = "v2_28", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
  pub use super::input_method_context::InputMethodContextExt;
  pub use super::install_missing_media_plugins_permission_request::InstallMissingMediaPluginsPermissionRequestExt;
  pub use super::navigation_policy_decision::NavigationPolicyDecisionExt;
  pub use super::notification::NotificationExt;
  pub use super::option_menu::OptionMenuExt;
  pub use super::permission_request::PermissionRequestExt;
  pub use super::plugin::PluginExt;
  pub use super::policy_decision::PolicyDecisionExt;
  pub use super::print_custom_widget::PrintCustomWidgetExt;
  pub use super::print_operation::PrintOperationExt;
  pub use super::response_policy_decision::ResponsePolicyDecisionExt;
  pub use super::security_manager::SecurityManagerExt;
  pub use super::settings::SettingsExt;
  pub use super::uri_request::URIRequestExt;
  pub use super::uri_response::URIResponseExt;
  pub use super::uri_scheme_request::URISchemeRequestExt;
  #[cfg(any(feature = "v2_36", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
  pub use super::uri_scheme_response::URISchemeResponseExt;
  pub use super::user_content_manager::UserContentManagerExt;
  pub use super::user_media_permission_request::UserMediaPermissionRequestExt;
  pub use super::user_message::UserMessageExt;
  pub use super::web_context::WebContextExt;
  pub use super::web_inspector::WebInspectorExt;
  pub use super::web_resource::WebResourceExt;
  pub use super::web_view::WebViewExt;
  pub use super::website_data_access_permission_request::WebsiteDataAccessPermissionRequestExt;
  pub use super::website_data_manager::WebsiteDataManagerExt;
  pub use super::website_policies::WebsitePoliciesExt;
  pub use super::window_properties::WindowPropertiesExt;
}
#[doc(hidden)]
pub mod builders {
  pub use super::automation_session::AutomationSessionBuilder;
  pub use super::color_chooser_request::ColorChooserRequestBuilder;
  pub use super::download::DownloadBuilder;
  pub use super::find_controller::FindControllerBuilder;
  pub use super::hit_test_result::HitTestResultBuilder;
  pub use super::print_custom_widget::PrintCustomWidgetBuilder;
  pub use super::print_operation::PrintOperationBuilder;
  pub use super::settings::SettingsBuilder;
  pub use super::uri_request::URIRequestBuilder;
  #[cfg(any(feature = "v2_36", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
  pub use super::uri_scheme_response::URISchemeResponseBuilder;
  pub use super::user_message::UserMessageBuilder;
  pub use super::web_context::WebContextBuilder;
  pub use super::web_view::WebViewBuilder;
  pub use super::website_data_manager::WebsiteDataManagerBuilder;
  pub use super::website_policies::WebsitePoliciesBuilder;
  pub use super::window_properties::WindowPropertiesBuilder;
}