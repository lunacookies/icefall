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
    builder.add_workspace_rule("editor.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("foreground", palette.base(BaseScale::Fg));

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

    builder.add_workspace_rule("editorWidget.background", palette.base(BaseScale::MiddleBg));

    builder.add_workspace_rule("sideBar.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rule(
        "sideBarSectionHeader.background",
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

    builder.add_workspace_rule(
        "editorGroupHeader.tabsBackground",
        palette.base(BaseScale::Bg),
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

    builder.add_workspace_rule("statusBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::DarkFg));

    builder.add_workspace_rule(
        "editor.selectionBackground",
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule("selection.background", palette.base(BaseScale::LightBg));

    builder.add_workspace_rule("input.background", palette.base(BaseScale::MiddleBg));
    builder.add_workspace_rule(
        "input.placeholderForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("editorCodeLens.foreground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("tab.border", palette.base(BaseScale::MiddleBg));
    builder.add_workspace_rule(
        "editorGroupHeader.border",
        palette.base(BaseScale::MiddleBg),
    );
    builder.add_workspace_rule(
        "editorGroupHeader.tabsBorder",
        palette.base(BaseScale::MiddleBg),
    );
    builder.add_workspace_rule("statusBar.border", palette.base(BaseScale::MiddleBg));

    builder.add_workspace_rule("input.border", palette.base(BaseScale::LightBg));
    builder.add_workspace_rule("editorGroup.border", palette.base(BaseScale::LightBg));
    builder.add_workspace_rule("sideBar.border", palette.base(BaseScale::LightBg));
    builder.add_workspace_rule("panel.border", palette.base(BaseScale::LightBg));
    builder.add_workspace_rule("editorWidget.border", palette.base(BaseScale::LightBg));

    builder.add_workspace_rule("focusBorder", palette.blue());
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Semantic("operator"),
            Semantic("builtinType"),
        ],
        palette.base(BaseScale::DimFg),
    );

    builder.add_rule(Semantic("macro"), palette.green());
    builder.add_rule(Semantic("operator.controlFlow"), palette.green());
    builder.add_rule(Semantic("typeParameter"), palette.green());
    builder.add_rule(Semantic("property"), palette.green());

    builder.add_rule(
        Semantic("comment"),
        (palette.base(BaseScale::BrightFg), FontStyle::Italic),
    );
    builder.add_rule(
        Semantic("comment.documentation"),
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
            Semantic("union.declaration"),
            Semantic("typeAlias.declaration"),
            Semantic("interface.declaration"),
            Semantic("namespace.declaration"),
        ],
        palette.base(BaseScale::BrightFg),
    );

    builder.add_rule(Semantic("string"), palette.cyan());

    builder.add_rules(
        &[
            Semantic("number"),
            Semantic("boolean"),
            Semantic("formatSpecifier"),
            Semantic("enumMember"),
            Semantic("*.constant"),
        ],
        palette.purple(),
    );

    builder.add_rules(
        &[Semantic("type"), Semantic("class"), Semantic("struct")],
        palette.blue(),
    );
    builder.add_rule(Semantic("enum"), palette.blue_2());

    builder.add_rule(Semantic("lifetime"), palette.green());

    builder.add_rule(Semantic("*.mutable"), FontStyle::Italic);
}
