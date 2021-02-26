<h1 align="center">Icefall</h1>
<h3 align="center">A cold, clear theme.</h3>

![](https://raw.githubusercontent.com/arzg/resources/master/icefall.png)

<h6 align="center"><em>The font in this screenshot is <a href="https://input.fontbureau.com">Input Sans</a>.</em></h6>

Icefall has subdued syntax highlighting,
opting instead to use its colours for semantic highlighting.
You can tell at a glance whether something is a function call,
so why waste colours on differentiating that?
You can’t tell whether `Foo` is an enum or a struct just by looking at it;
Icefall colours things depending on semantics, rather than syntax.

<img
    src="https://raw.githubusercontent.com/arzg/resources/master/icefall-semantic.png"
    width="200px"
    align="right">

Note how definitions are bright white, and how highlighting is semantic:

- `Boolean` is light blue, since it’s an enum
- `True` and `False` are purple, since they’re enum variants
- `Cat` is blue, since it’s a struct
- `Animal` is teal, since it’s a trait

This screenshot also uses a different font ([Input Serif][input]) for keywords
to further differentiate them.

The colour palette is taken almost entirely from the excellent [Iceberg][iceberg].

The VS Code theme file is generated using [mottle](https://github.com/arzg/mottle),
and the colour palette is generated with [tincture](https://github.com/arzg/tincture).
Take a look in `src` to see how theme generation is implemented.

[input]: https://input.fontbureau.com
[iceberg]: https://cocopon.github.io/iceberg.vim/
