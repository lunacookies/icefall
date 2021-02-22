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
    builder.add_workspace_rule("editorGutter.background", palette.base(BaseScale::MiddleBg));

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::MiddleBg),
    );

    builder.add_workspace_rule("sideBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule(
        "sideBarSectionHeader.background",
        palette.base(BaseScale::DarkBg),
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

    builder.add_workspace_rule("statusBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::DarkFg));

    builder.add_workspace_rule(
        "editor.selectionBackground",
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule("selection.background", palette.base(BaseScale::LightBg));

    builder.add_workspace_rule("editorCodeLens.foreground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.blue());

    builder.add_rules(
        &[Semantic("function"), Semantic("method")],
        palette.blue_2(),
    );

    builder.add_rules(
        &[Semantic("operator.controlFlow"), Semantic("typeParameter")],
        palette.green(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("typeAlias"),
            Semantic("builtInType"),
            Semantic("interface"),
        ],
        palette.blue(),
    );

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
        palette.orange(),
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

    builder.add_rule(Semantic("lifetime"), palette.green());
}
