//! Adapter module for notion-client types to our internal types
//!
//! This module provides conversion functions to transform types from the
//! notion-client crate into our internal representation, allowing us to reuse
//! our existing converters.

use notion_client::objects::rich_text::{RichText as ClientRichText, Annotations as ClientAnnotations};
use notion_client::objects::rich_text::{Link as ClientLink, Equation as ClientEquation};
use notion_client::objects::page::Color as ClientColor;

use crate::notion::text::{RichTextObject
