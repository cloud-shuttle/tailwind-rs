//! Trie-based parser lookup for fast class resolution
//!
//! This module implements a prefix trie that maps class name prefixes to parsers,
//! enabling O(1) or O(prefix_length) lookups instead of sequential O(n) checking.

use super::parsers::*;
use super::types::CssProperty;
use crate::transforms::TransformParser;
use crate::error::{Result, TailwindError};
use std::collections::HashMap;

/// Represents different parser types that can be stored in the trie
#[derive(Debug)]
pub enum ParserType {
    Arbitrary(ArbitraryParser),
    BasicTransforms(BasicTransformsParser),
    Scale(ScaleParser),
    MaskUtilities(MaskUtilitiesParser),
    Transform(TransformParser),
    Gradient(GradientParser),
    Effects(EffectsParser),
    Color(ColorParser),
    BackgroundProperties(BackgroundPropertiesParser),
    Typography(TypographyParser),
    Layout(LayoutParser),
    Flexbox(FlexboxParser),
    Sizing(SizingParser),
    Spacing(SpacingParser),
    Positioning(PositioningParser),
    AdvancedBorder(AdvancedBorderParser),
    Outline(OutlineParser),
    BorderRadius(BorderRadiusParser),
    Shadow(ShadowParser),
    Transition(TransitionParser),
    Ring(RingParser),
    Interactive(InteractiveParser),
    Animation(AnimationParser),
    AdvancedGrid(AdvancedGridParser),
    Svg(SvgParser),
    Prose(ProseParser),
    Divide(DivideParser),
    ObjectFit(ObjectFitParser),
    AccentColor(AccentColorParser),
    DataAttribute(DataAttributeParser),
    AdvancedColor(AdvancedColorParser),
    AdvancedSpacing(AdvancedSpacingParser),
    TransitionProperties(TransitionPropertiesParser),
    FractionalTransforms(FractionalTransformsParser),
    AspectRatio(AspectRatioParser),
    Columns(ColumnsParser),
    BreakControl(BreakControlParser),
    BoxUtilities(BoxUtilitiesParser),
    LayoutUtilities(LayoutUtilitiesParser),
    Overflow(OverflowParser),
    Overscroll(OverscrollParser),
    Position(PositionParser),
    Inset(InsetParser),
    Visibility(VisibilityParser),
    ZIndex(ZIndexParser),
    FlexBasis(FlexBasisParser),
    FlexDirection(FlexDirectionParser),
    FlexWrap(FlexWrapParser),
    Flex(FlexParser),
    FlexGrow(FlexGrowParser),
    FlexShrink(FlexShrinkParser),
    Order(OrderParser),
    GridTemplateColumns(GridTemplateColumnsParser),
    GridTemplateRows(GridTemplateRowsParser),
    GridColumn(GridColumnParser),
    GridRow(GridRowParser),
    GridAutoColumns(GridAutoColumnsParser),
    GridAutoFlow(GridAutoFlowParser),
    GridAutoRows(GridAutoRowsParser),
    Gap(GapParser),
    PlaceContent(PlaceContentParser),
    PlaceItems(PlaceItemsParser),
    PlaceSelf(PlaceSelfParser),
    JustifyContent(JustifyContentParser),
    JustifyItems(JustifyItemsParser),
    JustifySelf(JustifySelfParser),
    AlignContent(AlignContentParser),
    AlignItems(AlignItemsParser),
    AlignSelf(AlignSelfParser),
    Margin(MarginParser),
    Group(GroupParser),
    FilterUtilities(FilterUtilitiesParser),
    BackdropFilterUtilities(BackdropFilterUtilitiesParser),
    EffectsUtilities(EffectsUtilitiesParser),
    BorderUtilities(BorderUtilitiesParser),
    Accessibility(AccessibilityParser),
}

/// Trie node containing child nodes and an optional parser
#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    parser: Option<ParserType>,
    is_end_of_prefix: bool,
}

/// Prefix trie for fast parser lookup
#[derive(Debug)]
pub struct ParserTrie {
    root: TrieNode,
}

impl ParserTrie {
    /// Create a new empty parser trie
    pub fn new() -> Self {
        Self {
            root: TrieNode {
                children: HashMap::new(),
                parser: None,
                is_end_of_prefix: false,
            },
        }
    }

    /// Insert a prefix-parser mapping into the trie
    pub fn insert(&mut self, prefix: &str, parser: ParserType) {
        let mut node = &mut self.root;

        for ch in prefix.chars() {
            node = node.children.entry(ch).or_insert(TrieNode {
                children: HashMap::new(),
                parser: None,
                is_end_of_prefix: false,
            });
        }

        node.parser = Some(parser);
        node.is_end_of_prefix = true;
    }

    /// Find the best matching parser for a class name
    pub fn find_parser(&self, class: &str) -> Option<&ParserType> {
        let mut node = &self.root;
        let mut best_match = None;

        // Try to find the longest prefix match
        for ch in class.chars() {
            if let Some(child) = node.children.get(&ch) {
                node = child;
                if node.is_end_of_prefix {
                    best_match = node.parser.as_ref();
                }
            } else {
                break;
            }
        }

        best_match
    }

    /// Parse a class using the trie lookup
    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(parser) = self.find_parser(class) {
            match parser {
                ParserType::Arbitrary(p) => p.parse_class(class),
                ParserType::BasicTransforms(p) => p.parse_class(class),
                ParserType::Scale(p) => p.parse_class(class),
                ParserType::MaskUtilities(p) => p.parse_class(class),
                ParserType::Transform(p) => p.parse_class(class),
                ParserType::Gradient(p) => p.parse_class(class),
                ParserType::Effects(p) => p.parse_class(class),
                ParserType::Color(p) => p.parse_class(class),
                ParserType::BackgroundProperties(p) => p.parse_class(class),
                ParserType::Typography(p) => p.parse_class(class),
                ParserType::Layout(p) => p.parse_class(class),
                ParserType::Flexbox(p) => p.parse_class(class),
                ParserType::Sizing(p) => p.parse_class(class),
                ParserType::Spacing(p) => p.parse_class(class),
                ParserType::Positioning(p) => p.parse_class(class),
                ParserType::AdvancedBorder(p) => p.parse_class(class),
                ParserType::Outline(p) => p.parse_class(class),
                ParserType::BorderRadius(p) => p.parse_class(class),
                ParserType::Shadow(p) => p.parse_class(class),
                ParserType::Transition(p) => p.parse_class(class),
                ParserType::Ring(p) => p.parse_class(class),
                ParserType::Interactive(p) => p.parse_class(class),
                ParserType::Animation(p) => p.parse_class(class),
                ParserType::AdvancedGrid(p) => p.parse_class(class),
                ParserType::Svg(p) => p.parse_class(class),
                ParserType::Prose(p) => p.parse_class(class),
                ParserType::Divide(p) => p.parse_class(class),
                ParserType::ObjectFit(p) => p.parse_class(class),
                ParserType::AccentColor(p) => p.parse_class(class),
                ParserType::DataAttribute(p) => p.parse_class(class),
                ParserType::AdvancedColor(p) => p.parse_class(class),
                ParserType::AdvancedSpacing(p) => p.parse_class(class),
                ParserType::TransitionProperties(p) => p.parse_class(class),
                ParserType::FractionalTransforms(p) => p.parse_class(class),
                ParserType::AspectRatio(p) => p.parse_class(class),
                ParserType::Columns(p) => p.parse_class(class),
                ParserType::BreakControl(p) => p.parse_class(class),
                ParserType::BoxUtilities(p) => p.parse_class(class),
                ParserType::LayoutUtilities(p) => p.parse_class(class),
                ParserType::Overflow(p) => p.parse_class(class),
                ParserType::Overscroll(p) => p.parse_class(class),
                ParserType::Position(p) => p.parse_class(class),
                ParserType::Inset(p) => p.parse_class(class),
                ParserType::Visibility(p) => p.parse_class(class),
                ParserType::ZIndex(p) => p.parse_class(class),
                ParserType::FlexBasis(p) => p.parse_class(class),
                ParserType::FlexDirection(p) => p.parse_class(class),
                ParserType::FlexWrap(p) => p.parse_class(class),
                ParserType::Flex(p) => p.parse_class(class),
                ParserType::FlexGrow(p) => p.parse_class(class),
                ParserType::FlexShrink(p) => p.parse_class(class),
                ParserType::Order(p) => p.parse_class(class),
                ParserType::GridTemplateColumns(p) => p.parse_class(class),
                ParserType::GridTemplateRows(p) => p.parse_class(class),
                ParserType::GridColumn(p) => p.parse_class(class),
                ParserType::GridRow(p) => p.parse_class(class),
                ParserType::GridAutoColumns(p) => p.parse_class(class),
                ParserType::GridAutoFlow(p) => p.parse_class(class),
                ParserType::GridAutoRows(p) => p.parse_class(class),
                ParserType::Gap(p) => p.parse_class(class),
                ParserType::PlaceContent(p) => p.parse_class(class),
                ParserType::PlaceItems(p) => p.parse_class(class),
                ParserType::PlaceSelf(p) => p.parse_class(class),
                ParserType::JustifyContent(p) => p.parse_class(class),
                ParserType::JustifyItems(p) => p.parse_class(class),
                ParserType::JustifySelf(p) => p.parse_class(class),
                ParserType::AlignContent(p) => p.parse_class(class),
                ParserType::AlignItems(p) => p.parse_class(class),
                ParserType::AlignSelf(p) => p.parse_class(class),
                ParserType::Margin(p) => p.parse_class(class),
                ParserType::Group(p) => p.parse_class(class),
                ParserType::FilterUtilities(p) => p.parse_class(class),
                ParserType::BackdropFilterUtilities(p) => p.parse_class(class),
                ParserType::EffectsUtilities(p) => p.parse_class(class),
                ParserType::BorderUtilities(p) => p.parse_class(class),
                ParserType::Accessibility(p) => p.parse_class(class),
            }
        } else {
            None
        }
    }
}

impl Default for ParserTrie {
    fn default() -> Self {
        Self::new()
    }
}

impl TrieNode {
    /// Create a new trie node
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            parser: None,
            is_end_of_prefix: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_basic() {
        let mut trie = ParserTrie::new();

        // Insert a test parser
        let parser = ColorParser::new();
        trie.insert("text-", ParserType::Color(parser));

        // Test lookup
        assert!(trie.find_parser("text-red-500").is_some());
        assert!(trie.find_parser("bg-red-500").is_none());
    }

    #[test]
    fn test_trie_parsing() {
        let mut trie = ParserTrie::new();

        // Insert color parser
        let parser = ColorParser::new();
        trie.insert("text-", ParserType::Color(parser));

        // Test parsing
        let result = trie.parse_class("text-red-500");
        assert!(result.is_some());

        let result = trie.parse_class("unknown-class");
        assert!(result.is_none());
    }
}
