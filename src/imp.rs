use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "editorLineNumber.foreground",
        palette.base(BaseScale::BarelyVisibleFg),
    );
    builder.add_workspace_rule(
        "editorLineNumber.activeForeground",
        palette.base(BaseScale::Fg),
    );
    builder.add_workspace_rule("editorGutter.background", palette.base(BaseScale::MiddleBg));

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::MiddleBg),
    );

    builder.add_workspace_rules(
        &[
            "editor.rangeHighlightBackground",
            "editorOverviewRuler.rangeHighlightForeground",
        ],
        palette.base(BaseScale::LightBg),
    );

    builder.add_workspace_rules(
        &["minimapSlider.background", "scrollbarSlider.background"],
        (palette.base(BaseScale::LightBg), 0x40),
    );
    builder.add_workspace_rules(
        &[
            "minimapSlider.hoverBackground",
            "scrollbarSlider.hoverBackground",
        ],
        (palette.base(BaseScale::LightBg), 0x60),
    );
    builder.add_workspace_rules(
        &[
            "minimapSlider.activeBackground",
            "scrollbarSlider.activeBackground",
        ],
        (palette.base(BaseScale::LightBg), 0xA0),
    );

    builder.add_workspace_rules(
        &[
            "editor.findMatchHighlightBackground",
            "editor.findMatchBackground",
            "minimap.findMatchHighlight",
            "editorOverviewRuler.findMatchForeground",
            "peekViewEditor.matchHighlightBackground",
        ],
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule("editor.findMatchBorder", palette.base(BaseScale::DarkFg));

    builder.add_workspace_rule("editorWidget.background", palette.base(BaseScale::MiddleBg));

    builder.add_workspace_rule("list.focusForeground", palette.base(BaseScale::Fg));
    builder.add_workspace_rules(
        &[
            "list.activeSelectionForeground",
            "list.inactiveSelectionForeground",
            "list.highlightForeground",
        ],
        palette.base(BaseScale::BrightFg),
    );

    builder.add_workspace_rules(
        &[
            "list.focusBackground",
            "list.activeSelectionBackground",
            "peekViewResult.selectionBackground",
        ],
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule(
        "list.inactiveSelectionBackground",
        palette.base(BaseScale::MiddleBg),
    );
    builder.add_workspace_rule("list.hoverBackground", palette.base(BaseScale::Bg));

    builder.add_workspace_rules(
        &["sideBar.background", "sideBarSectionHeader.background"],
        palette.base(BaseScale::Bg),
    );
    builder.add_workspace_rule(
        "sideBarSectionHeader.foreground",
        palette.base(BaseScale::BrightFg),
    );

    builder.add_workspace_rule("activityBar.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rule("activityBar.foreground", palette.base(BaseScale::BrightFg));
    builder.add_workspace_rule(
        "activityBar.inactiveForeground",
        palette.base(BaseScale::BarelyVisibleFg),
    );

    builder.add_workspace_rules(
        &[
            "editorGroupHeader.tabsBackground",
            "editorGroupHeader.noTabsBackground",
        ],
        palette.base(BaseScale::DarkBg),
    );
    builder.add_workspace_rule("tab.inactiveBackground", palette.base(BaseScale::Bg));
    builder.add_workspace_rule("tab.inactiveForeground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("tab.activeBackground", palette.base(BaseScale::MiddleBg));
    builder.add_workspace_rule("tab.activeForeground", palette.base(BaseScale::BrightFg));
    builder.add_workspace_rule(
        "tab.unfocusedInactiveBackground",
        palette.base(BaseScale::Bg),
    );
    builder.add_workspace_rule(
        "tab.unfocusedInactiveForeground",
        palette.base(BaseScale::BarelyVisibleFg),
    );
    builder.add_workspace_rule("tab.unfocusedActiveBackground", palette.base(BaseScale::Bg));
    builder.add_workspace_rule(
        "tab.unfocusedActiveForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rules(
        &[
            "statusBar.background",
            "statusBar.debuggingBackground",
            "statusBar.noFolderBackground",
        ],
        palette.base(BaseScale::DarkBg),
    );
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("statusBar.debuggingForeground", palette.orange());

    builder.add_workspace_rules(
        &["titleBar.activeBackground", "titleBar.inactiveBackground"],
        palette.base(BaseScale::DarkBg),
    );
    builder.add_workspace_rule("titleBar.activeForeground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule(
        "titleBar.inactiveForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rules(
        &["editor.selectionBackground", "minimap.selectionHighlight"],
        palette.base(BaseScale::LightBg),
    );

    // Text selections outside of the editor have their colour darkened,
    // so we compensate by choosing a ridiculously bright selection colour.
    builder.add_workspace_rule("selection.background", palette.base(BaseScale::DimFg));

    builder.add_workspace_rules(
        &[
            "editor.hoverHighlightBackground",
            "editor.wordHighlightBackground",
            "editorOverviewRuler.wordHighlightForeground",
            "editor.wordHighlightStrongBackground",
            "editorOverviewRuler.wordHighlightStrongForeground",
            "editor.symbolHighlightBackground",
            "editor.selectionHighlightBackground",
            "editorOverviewRuler.selectionHighlightForeground",
        ],
        palette.base(BaseScale::LightBg),
    );

    builder.add_workspace_rule("input.background", palette.base(BaseScale::MiddleBg));
    builder.add_workspace_rule(
        "input.placeholderForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("breadcrumb.foreground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("editorCodeLens.foreground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("diffEditor.insertedTextBackground", (palette.green(), 0x20));
    builder.add_workspace_rule("diffEditor.removedTextBackground", (palette.red(), 0x20));

    builder.add_workspace_rules(
        &[
            "gitDecoration.addedResourceForeground",
            "gitDecoration.untrackedResourceForeground",
            "editorGutter.addedBackground",
            "minimapGutter.addedBackground",
            "editorOverviewRuler.addedForeground",
        ],
        palette.green(),
    );
    builder.add_workspace_rules(
        &[
            "gitDecoration.deletedResourceForeground",
            "gitDecoration.stageDeletedResourceForeground",
            "editorGutter.deletedBackground",
            "minimapGutter.deletedBackground",
            "editorOverviewRuler.deletedForeground",
        ],
        palette.red(),
    );
    builder.add_workspace_rules(
        &[
            "gitDecoration.modifiedResourceForeground",
            "gitDecoration.stageModifiedResourceForeground",
            "editorGutter.modifiedBackground",
            "minimapGutter.modifiedBackground",
            "editorOverviewRuler.modifiedForeground",
        ],
        palette.orange(),
    );
    builder.add_workspace_rule(
        "gitDecoration.conflictingResourceForeground",
        palette.purple(),
    );
    builder.add_workspace_rule(
        "gitDecoration.ignoredResourceForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rules(
        &[
            "errorForeground",
            "editorError.foreground",
            "list.errorForeground",
            "inputValidation.errorForeground",
            "inputValidation.errorBorder",
            "minimap.errorHighlight",
            "editorOverviewRuler.errorForeground",
        ],
        palette.red(),
    );
    builder.add_workspace_rules(
        &[
            "editorWarning.foreground",
            "list.warningForeground",
            "inputValidation.warningForeground",
            "inputValidation.warningBorder",
            "minimap.warningHighlight",
            "editorOverviewRuler.warningForeground",
        ],
        palette.orange(),
    );
    builder.add_workspace_rule(
        "inputValidation.errorBackground",
        palette.base(BaseScale::MiddleBg),
    );

    builder.add_workspace_rule("peekViewEditor.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("peekViewResult.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rule(
        "peekViewTitle.background",
        palette.base(BaseScale::MiddleBg),
    );
    builder.add_workspace_rule(
        "peekViewResult.fileForeground",
        palette.base(BaseScale::BrightFg),
    );
    builder.add_workspace_rule("peekViewResult.lineForeground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule(
        "peekViewTitleLabel.foreground",
        palette.base(BaseScale::BrightFg),
    );
    builder.add_workspace_rule(
        "peekViewTitleDescription.foreground",
        palette.base(BaseScale::DarkFg),
    );
    builder.add_workspace_rule("peekView.border", palette.base(BaseScale::BarelyVisibleFg));

    builder.add_workspace_rules(
        &["badge.background", "activityBarBadge.background"],
        palette.blue_2(),
    );
    builder.add_workspace_rules(
        &["badge.foreground", "activityBarBadge.foreground"],
        palette.base(BaseScale::Bg),
    );

    builder.add_workspace_rule("textLink.foreground", palette.blue());
    builder.add_workspace_rule("textLink.activeForeground", palette.blue_2());

    builder.add_workspace_rule("terminal.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("terminal.ansiBlack", palette.base(BaseScale::LightBg));
    builder.add_workspace_rule("terminal.ansiBrightBlack", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("terminal.ansiRed", palette.red());
    builder.add_workspace_rule("terminal.ansiBrightRed", palette.red());
    builder.add_workspace_rule("terminal.ansiGreen", palette.green());
    builder.add_workspace_rule("terminal.ansiBrightGreen", palette.green());
    builder.add_workspace_rule("terminal.ansiYellow", palette.orange());
    builder.add_workspace_rule("terminal.ansiBrightYellow", palette.orange());
    builder.add_workspace_rule("terminal.ansiBlue", palette.blue());
    builder.add_workspace_rule("terminal.ansiBrightBlue", palette.blue());
    builder.add_workspace_rule("terminal.ansiMagenta", palette.purple());
    builder.add_workspace_rule("terminal.ansiBrightMagenta", palette.purple());
    builder.add_workspace_rule("terminal.ansiCyan", palette.cyan());
    builder.add_workspace_rule("terminal.ansiBrightCyan", palette.cyan());
    builder.add_workspace_rule("terminal.ansiWhite", palette.base(BaseScale::Fg));
    builder.add_workspace_rule(
        "terminal.ansiBrightWhite",
        palette.base(BaseScale::BrightFg),
    );

    builder.add_workspace_rules(
        &["editorCursor.foreground", "terminalCursor.foreground"],
        palette.base(BaseScale::BrightFg),
    );
    builder.add_workspace_rules(
        &["editorCursor.background", "terminalCursor.background"],
        palette.base(BaseScale::DarkBg),
    );

    builder.add_workspace_rules(
        &["editorIndentGuide.background", "tree.indentGuidesStroke"],
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule(
        "editorIndentGuide.activeBackground",
        palette.base(BaseScale::BarelyVisibleFg),
    );

    builder.add_workspace_rules(
        &[
            "tab.border",
            "editorGroupHeader.border",
            "editorGroupHeader.tabsBorder",
            "statusBar.border",
        ],
        palette.base(BaseScale::MiddleBg),
    );

    builder.add_workspace_rules(
        &[
            "input.border",
            "editorGroup.border",
            "sideBar.border",
            "activityBar.border",
            "panel.border",
            "editorWidget.border",
            "editorOverviewRuler.border",
            "editorRuler.foreground",
        ],
        palette.base(BaseScale::LightBg),
    );

    builder.add_workspace_rule("focusBorder", palette.blue());
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Semantic("operator"),
            Textmate("punctuation.separator"),
            Semantic("builtinType"),
            Textmate("keyword"),
            Textmate("storage"),
            Textmate("variable.language.self"),
        ],
        palette.base(BaseScale::DimFg),
    );

    builder.add_rules(
        &[
            Semantic("macro"),
            Textmate("keyword.control.directive"),
            Textmate("keyword.preprocessor"),
            Textmate("punctuation.separator.hash.cs"),
            Semantic("operator.controlFlow"),
            Semantic("typeParameter"),
            Semantic("parameter"),
            Textmate("variable.parameter"),
            Textmate("entity.name.variable.parameter"),
        ],
        palette.orange(),
    );
    builder.add_rule(Semantic("lifetime"), (palette.orange(), FontStyle::Italic));

    builder.add_rules(
        &[Semantic("comment"), Textmate("comment")],
        (palette.base(BaseScale::BrightFg), FontStyle::Italic),
    );
    builder.add_rules(
        &[
            Semantic("comment.documentation"),
            Textmate("comment.block.documentation"),
            Textmate("comment.block.javadoc"),
        ],
        palette.base(BaseScale::BrightFg),
    );

    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
            Semantic("type.declaration"),
            Semantic("class.declaration"),
            Semantic("struct.declaration"),
            Semantic("enum.declaration"),
            Semantic("enumMember.declaration"),
            Semantic("union.declaration"),
            Semantic("typeAlias.declaration"),
            Semantic("interface.declaration"),
            Semantic("namespace.declaration"),
            Textmate("entity.name.type"),
            Textmate("entity.name.package"),
            Textmate("entity.name.function.go"),
            Textmate("entity.name.function.python"),
            Textmate("variable.other.enummember"),
        ],
        palette.base(BaseScale::BrightFg),
    );

    builder.add_rules(&[Semantic("string"), Textmate("string")], palette.cyan());

    builder.add_rules(
        &[
            Semantic("number"),
            Semantic("boolean"),
            Semantic("formatSpecifier"),
            Semantic("enumMember"),
            Semantic("*.constant"),
            Textmate("constant"),
            Textmate("support.constant"),
            Textmate("entity.name.variable.preprocessor"),
        ],
        palette.purple(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Textmate("entity.name.type.rust"),
            Textmate("storage.type.cs"),
        ],
        palette.blue(),
    );
    builder.add_rule(Semantic("enum"), palette.blue_2());
    builder.add_rule(Semantic("interface"), palette.cyan());

    builder.add_rules(
        &[
            Semantic("property"),
            Textmate("entity.name.variable.field"),
            Textmate("variable.other.object.property"),
            Textmate("support.type.property-name"),
            Textmate("entity.other.attribute-name.html"),
        ],
        palette.green(),
    );

    builder.add_rules(
        &[
            Semantic("namespace"),
            Textmate("entity.name.namespace"),
            Textmate("entity.name.type.namespace"),
        ],
        palette.base(BaseScale::DimFg),
    );

    builder.add_rule(Textmate("entity.name.tag"), palette.blue());
    builder.add_rules(
        &[
            Textmate("punctuation.definition.tag"),
            Textmate("punctuation.section.embedded.begin.hugo"),
            Textmate("punctuation.section.embedded.end.hugo"),
        ],
        palette.base(BaseScale::DarkFg),
    );

    builder.add_rule(Textmate("entity.other.attribute-name"), palette.blue());

    builder.add_rules(
        &[
            Semantic("*.attribute"),
            Textmate("entity.name.function.decorator"),
        ],
        palette.base(BaseScale::DarkFg),
    );

    builder.add_rule(
        Textmate("markup.heading"),
        palette.base(BaseScale::BrightFg),
    );
    builder.add_rule(Textmate("markup.bold"), FontStyle::Bold);
    builder.add_rule(Textmate("markup.italic"), FontStyle::Italic);
    builder.add_rules(
        &[
            Textmate("string.other.link.title.markdown"),
            Textmate("constant.other.reference.link.markdown"),
        ],
        palette.base(BaseScale::Fg),
    );
    builder.add_rules(
        &[
            Textmate("punctuation.definition.heading.markdown"),
            Textmate("punctuation.definition.bold.markdown"),
            Textmate("punctuation.definition.italic.markdown"),
            Textmate("punctuation.definition.raw.markdown"),
            Textmate("punctuation.definition.string.begin.markdown"),
            Textmate("punctuation.definition.string.end.markdown"),
            Textmate("punctuation.definition.metadata.markdown"),
            Textmate("punctuation.definition.list.begin.markdown"),
            Textmate("punctuation.definition.constant.markdown"),
            Textmate("punctuation.definition.constant.begin.markdown"),
            Textmate("punctuation.definition.constant.end.markdown"),
            Textmate("punctuation.separator.key-value.markdown"),
            Textmate("punctuation.definition.markdown"),
            Textmate("fenced_code.block.language.markdown"),
            Textmate("meta.separator.markdown"),
        ],
        palette.blue(),
    );

    builder.add_rule(
        Textmate("meta.scope.message.git-commit"),
        palette.base(BaseScale::BrightFg),
    );

    // Over 50 characters, the recommended limit.
    builder.add_rule(
        Textmate("invalid.deprecated.line-too-long.git-commit"),
        palette.orange(),
    );

    // Over 72 characters, the hard limit.
    builder.add_rule(
        Textmate("invalid.illegal.line-too-long.git-commit"),
        palette.red(),
    );

    builder.add_rule(Textmate("markup.inserted"), palette.green());
    builder.add_rule(Textmate("markup.deleted"), palette.red());
    builder.add_rule(Textmate("markup.changed"), palette.orange());
    builder.add_rules(
        &[
            Textmate("meta.diff.range"),
            Textmate("meta.diff.header"),
            Textmate("meta.diff.index"),
            Textmate("comment.line.number-sign.git-commit"),
        ],
        palette.base(BaseScale::DarkFg),
    );

    builder.add_rule(Textmate("magit.header"), palette.base(BaseScale::BrightFg));
    builder.add_rule(Textmate("magit.subheader"), palette.blue());
    builder.add_rule(Semantic("magit-ref-name"), palette.cyan());
    builder.add_rule(Semantic("magit-remote-ref-name"), palette.green());
    builder.add_rule(Semantic("magit-tag-name"), palette.blue_2());
    builder.add_rule(Textmate("magit.entity"), palette.purple());

    builder.add_rule(
        Semantic("unresolvedReference"),
        (palette.red(), FontStyle::Bold),
    );

    builder.add_rule(Semantic("*.mutable"), FontStyle::Italic);
    builder.add_rule(Semantic("*.consuming"), FontStyle::Underline);
}
