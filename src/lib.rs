#![no_std]
#![deny(rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(improper_ctypes, improper_ctypes_definitions)]
#![deny(clippy::all)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", deny(warnings))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! The cross platform cursor icon type.

// This file contains a portion of the CSS Basic User Interface Module Level 3
// specification. In particular, the names for the cursor from the #cursor
// section and documentation for some of the variants were taken.
//
// The original document is https://www.w3.org/TR/css-ui-3/#cursor.
// Copyright © 2018 W3C® (MIT, ERCIM, Keio, Beihang)
//
// These documents were used under the terms of the following license. This W3C
// license as well as the W3C short notice apply to the `CursorIcon` enum's
// variants and documentation attached to them.

// --------- BEGGINING OF W3C LICENSE
// --------------------------------------------------------------
//
// License
//
// By obtaining and/or copying this work, you (the licensee) agree that you have
// read, understood, and will comply with the following terms and conditions.
//
// Permission to copy, modify, and distribute this work, with or without
// modification, for any purpose and without fee or royalty is hereby granted,
// provided that you include the following on ALL copies of the work or portions
// thereof, including modifications:
//
// - The full text of this NOTICE in a location viewable to users of the
//   redistributed or derivative work.
// - Any pre-existing intellectual property disclaimers, notices, or terms and
//   conditions. If none exist, the W3C Software and Document Short Notice
//   should be included.
// - Notice of any changes or modifications, through a copyright statement on
//   the new code or document such as "This software or document includes
//   material copied from or derived from [title and URI of the W3C document].
//   Copyright © [YEAR] W3C® (MIT, ERCIM, Keio, Beihang)."
//
// Disclaimers
//
// THIS WORK IS PROVIDED "AS IS," AND COPYRIGHT HOLDERS MAKE NO REPRESENTATIONS
// OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES
// OF MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
// THE SOFTWARE OR DOCUMENT WILL NOT INFRINGE ANY THIRD PARTY PATENTS,
// COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
//
// COPYRIGHT HOLDERS WILL NOT BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL OR
// CONSEQUENTIAL DAMAGES ARISING OUT OF ANY USE OF THE SOFTWARE OR DOCUMENT.
//
// The name and trademarks of copyright holders may NOT be used in advertising
// or publicity pertaining to the work without specific, written prior
// permission. Title to copyright in this work will at all times remain with
// copyright holders.
//
// --------- END OF W3C LICENSE
// --------------------------------------------------------------------

// --------- BEGGINING OF W3C SHORT NOTICE
// ---------------------------------------------------------
//
// winit: https://github.com/rust-windowing/cursor-icon
//
// Copyright © 2023 World Wide Web Consortium, (Massachusetts Institute of
// Technology, European Research Consortium for Informatics and Mathematics,
// Keio University, Beihang). All Rights Reserved. This work is distributed
// under the W3C® Software License [1] in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.
//
// [1] http://www.w3.org/Consortium/Legal/copyright-software
//
// --------- END OF W3C SHORT NOTICE
// --------------------------------------------------------------

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

/// Describes the appearance of the (usually mouse) cursor icon.
///
/// The names are taken from the CSS W3C specification:
/// <https://w3c.github.io/csswg-drafts/css-ui/#cursor>
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CursorIcon {
    /// The platform-dependent default cursor. Often rendered as arrow.
    #[default]
    Default,

    /// A context menu is available for the object under the cursor. Often
    /// rendered as an arrow with a small menu-like graphic next to it.
    ContextMenu,

    /// Help is available for the object under the cursor. Often rendered as a
    /// question mark or a balloon.
    Help,

    /// The cursor is a pointer that indicates a link. Often rendered as the
    /// backside of a hand with the index finger extended.
    Pointer,

    /// A progress indicator. The program is performing some processing, but is
    /// different from [`CursorIcon::Wait`] in that the user may still interact with the
    /// program.
    Progress,

    /// Indicates that the program is busy and the user should wait. Often
    /// rendered as a watch or hourglass.
    Wait,

    /// Indicates that a cell or set of cells may be selected. Often rendered as
    /// a thick plus-sign with a dot in the middle.
    Cell,

    /// A simple crosshair (e.g., short line segments resembling a "+" sign).
    /// Often used to indicate a two dimensional bitmap selection mode.
    Crosshair,

    /// Indicates text that may be selected. Often rendered as an I-beam.
    Text,

    /// Indicates vertical-text that may be selected. Often rendered as a
    /// horizontal I-beam.
    VerticalText,

    /// Indicates an alias of/shortcut to something is to be created. Often
    /// rendered as an arrow with a small curved arrow next to it.
    Alias,

    /// Indicates something is to be copied. Often rendered as an arrow with a
    /// small plus sign next to it.
    Copy,

    /// Indicates something is to be moved.
    Move,

    /// Indicates that the dragged item cannot be dropped at the current cursor
    /// location. Often rendered as a hand or pointer with a small circle with a
    /// line through it.
    NoDrop,

    /// Indicates that the requested action will not be carried out. Often
    /// rendered as a circle with a line through it.
    NotAllowed,

    /// Indicates that something can be grabbed (dragged to be moved). Often
    /// rendered as the backside of an open hand.
    Grab,

    /// Indicates that something is being grabbed (dragged to be moved). Often
    /// rendered as the backside of a hand with fingers closed mostly out of
    /// view.
    Grabbing,

    /// The east border to be moved.
    EResize,

    /// The north border to be moved.
    NResize,

    /// The north-east corner to be moved.
    NeResize,

    /// The north-west corner to be moved.
    NwResize,

    /// The south border to be moved.
    SResize,

    /// The south-east corner to be moved.
    SeResize,

    /// The south-west corner to be moved.
    SwResize,

    /// The west border to be moved.
    WResize,

    /// The east and west borders to be moved.
    EwResize,

    /// The south and north borders to be moved.
    NsResize,

    /// The north-east and south-west corners to be moved.
    NeswResize,

    /// The north-west and south-east corners to be moved.
    NwseResize,

    /// Indicates that the item/column can be resized horizontally. Often
    /// rendered as arrows pointing left and right with a vertical bar
    /// separating them.
    ColResize,

    /// Indicates that the item/row can be resized vertically. Often rendered as
    /// arrows pointing up and down with a horizontal bar separating them.
    RowResize,

    /// Indicates that the something can be scrolled in any direction. Often
    /// rendered as arrows pointing up, down, left, and right with a dot in the
    /// middle.
    AllScroll,

    /// Indicates that something can be zoomed in. Often rendered as a
    /// magnifying glass with a "+" in the center of the glass.
    ZoomIn,

    /// Indicates that something can be zoomed in. Often rendered as a
    /// magnifying glass with a "-" in the center of the glass.
    ZoomOut,
}

impl CursorIcon {
    /// The name of the cursor icon as defined in w3c standard.
    ///
    /// This name most of the time could be passed as is to cursor loading
    /// libraries on X11/Wayland and could be used as-is on web.
    pub fn name(&self) -> &'static str {
        match self {
            CursorIcon::Default => "default",
            CursorIcon::ContextMenu => "context-menu",
            CursorIcon::Help => "help",
            CursorIcon::Pointer => "pointer",
            CursorIcon::Progress => "progress",
            CursorIcon::Wait => "wait",
            CursorIcon::Cell => "cell",
            CursorIcon::Crosshair => "crosshair",
            CursorIcon::Text => "text",
            CursorIcon::VerticalText => "vertical-text",
            CursorIcon::Alias => "alias",
            CursorIcon::Copy => "copy",
            CursorIcon::Move => "move",
            CursorIcon::NoDrop => "no-drop",
            CursorIcon::NotAllowed => "not-allowed",
            CursorIcon::Grab => "grab",
            CursorIcon::Grabbing => "grabbing",
            CursorIcon::EResize => "e-resize",
            CursorIcon::NResize => "n-resize",
            CursorIcon::NeResize => "ne-resize",
            CursorIcon::NwResize => "nw-resize",
            CursorIcon::SResize => "s-resize",
            CursorIcon::SeResize => "se-resize",
            CursorIcon::SwResize => "sw-resize",
            CursorIcon::WResize => "w-resize",
            CursorIcon::EwResize => "ew-resize",
            CursorIcon::NsResize => "ns-resize",
            CursorIcon::NeswResize => "nesw-resize",
            CursorIcon::NwseResize => "nwse-resize",
            CursorIcon::ColResize => "col-resize",
            CursorIcon::RowResize => "row-resize",
            CursorIcon::AllScroll => "all-scroll",
            CursorIcon::ZoomIn => "zoom-in",
            CursorIcon::ZoomOut => "zoom-out",
        }
    }
}

impl core::str::FromStr for CursorIcon {
    type Err = CursorIconParseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "default" => Ok(CursorIcon::Default),
            "context-menu" => Ok(CursorIcon::ContextMenu),
            "help" => Ok(CursorIcon::Help),
            "pointer" => Ok(CursorIcon::Pointer),
            "progress" => Ok(CursorIcon::Progress),
            "wait" => Ok(CursorIcon::Wait),
            "cell" => Ok(CursorIcon::Cell),
            "crosshair" => Ok(CursorIcon::Crosshair),
            "text" => Ok(CursorIcon::Text),
            "vertical-text" => Ok(CursorIcon::VerticalText),
            "alias" => Ok(CursorIcon::Alias),
            "copy" => Ok(CursorIcon::Copy),
            "move" => Ok(CursorIcon::Move),
            "no-drop" => Ok(CursorIcon::NoDrop),
            "not-allowed" => Ok(CursorIcon::NotAllowed),
            "grab" => Ok(CursorIcon::Grab),
            "grabbing" => Ok(CursorIcon::Grabbing),
            "e-resize" => Ok(CursorIcon::EResize),
            "n-resize" => Ok(CursorIcon::NResize),
            "ne-resize" => Ok(CursorIcon::NeResize),
            "nw-resize" => Ok(CursorIcon::NwResize),
            "s-resize" => Ok(CursorIcon::SResize),
            "se-resize" => Ok(CursorIcon::SeResize),
            "sw-resize" => Ok(CursorIcon::SwResize),
            "w-resize" => Ok(CursorIcon::WResize),
            "ew-resize" => Ok(CursorIcon::EwResize),
            "ns-resize" => Ok(CursorIcon::NsResize),
            "nesw-resize" => Ok(CursorIcon::NeswResize),
            "nwse-resize" => Ok(CursorIcon::NwseResize),
            "col-resize" => Ok(CursorIcon::ColResize),
            "row-resize" => Ok(CursorIcon::RowResize),
            "all-scroll" => Ok(CursorIcon::AllScroll),
            "zoom-in" => Ok(CursorIcon::ZoomIn),
            "zoom-out" => Ok(CursorIcon::ZoomOut),
            _ => Err(CursorIconParseError),
        }
    }
}

/// An error which could be returned when parsing [`CursorIcon`].
#[derive(Debug, PartialEq, Eq)]
pub struct CursorIconParseError;

impl core::fmt::Display for CursorIconParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("failed to parse cursor icon")
    }
}
