// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod action;
pub use self::action::Action;

mod component;
pub use self::component::Component;

mod document;
pub use self::document::Document;

mod editable_text;
pub use self::editable_text::EditableText;

mod gobject_accessible;
pub use self::gobject_accessible::GObjectAccessible;

mod hyperlink;
pub use self::hyperlink::Hyperlink;

mod hyperlink_impl;
pub use self::hyperlink_impl::HyperlinkImpl;

mod hypertext;
pub use self::hypertext::Hypertext;

mod image;
pub use self::image::Image;

mod misc;
pub use self::misc::Misc;

mod no_op_object;
pub use self::no_op_object::NoOpObject;

mod no_op_object_factory;
pub use self::no_op_object_factory::NoOpObjectFactory;

mod object;
pub use self::object::Object;

mod object_factory;
pub use self::object_factory::ObjectFactory;

mod plug;
pub use self::plug::Plug;

mod registry;
pub use self::registry::Registry;

mod relation;
pub use self::relation::Relation;

mod relation_set;
pub use self::relation_set::RelationSet;

mod selection;
pub use self::selection::Selection;

mod socket;
pub use self::socket::Socket;

mod state_set;
pub use self::state_set::StateSet;

mod streamable_content;
pub use self::streamable_content::StreamableContent;

mod table;
pub use self::table::Table;

mod table_cell;
pub use self::table_cell::TableCell;

mod text;
pub use self::text::Text;

mod util;
pub use self::util::Util;

mod value;
pub use self::value::Value;

mod window;
pub use self::window::Window;

mod range;
pub use self::range::Range;

mod rectangle;
pub use self::rectangle::Rectangle;

mod text_range;
pub use self::text_range::TextRange;

mod enums;
pub use self::enums::CoordType;
pub use self::enums::Layer;
pub use self::enums::RelationType;
pub use self::enums::Role;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::enums::ScrollType;
pub use self::enums::StateType;
pub use self::enums::TextAttribute;
pub use self::enums::TextBoundary;
pub use self::enums::TextClipType;
pub use self::enums::TextGranularity;
pub use self::enums::ValueType;

mod flags;
pub use self::flags::HyperlinkStateFlags;

mod alias;
pub use self::alias::State;

#[doc(hidden)]
pub mod traits {
    pub use super::action::AtkActionExt;
    pub use super::component::ComponentExt;
    pub use super::document::DocumentExt;
    pub use super::editable_text::EditableTextExt;
    pub use super::gobject_accessible::GObjectAccessibleExt;
    pub use super::hyperlink::HyperlinkExt;
    pub use super::hyperlink_impl::HyperlinkImplExt;
    pub use super::hypertext::HypertextExt;
    pub use super::image::AtkImageExt;
    pub use super::misc::AtkMiscExt;
    pub use super::object::AtkObjectExt;
    pub use super::object_factory::ObjectFactoryExt;
    pub use super::plug::AtkPlugExt;
    pub use super::registry::RegistryExt;
    pub use super::relation::RelationExt;
    pub use super::relation_set::RelationSetExt;
    pub use super::selection::SelectionExt;
    pub use super::socket::AtkSocketExt;
    pub use super::state_set::StateSetExt;
    pub use super::streamable_content::StreamableContentExt;
    pub use super::table::TableExt;
    pub use super::table_cell::TableCellExt;
    pub use super::text::TextExt;
    pub use super::value::ValueExt;
    pub use super::window::AtkWindowExt;
}
