<center>

# Icefall

![](https://raw.githubusercontent.com/arzg/resources/master/icefall.png)

_The font in this screenshot is [Input Sans](https://input.fontbureau.com)._

</center>

A cold, clear theme.
Icefall has subdued syntax highlighting,
opting instead to use its colours for semantic highlighting.
You can tell at a glance whether something is a function call,
so why waste colours on differentiating that?
You can’t tell whether `Foo` is an enum or a struct just by looking at it;
Icefall colours things depending on semantics, rather than syntax.

The colour palette is taken almost entirely from the excellent [Iceberg][iceberg].

The VS Code theme file is generated using [mottle](https://github.com/arzg/mottle),
and the colour palette is generated with [tincture](https://github.com/arzg/tincture).
Take a look in `src` to see how theme generation is implemented.

[iceberg]: https://cocopon.github.io/iceberg.vim/
