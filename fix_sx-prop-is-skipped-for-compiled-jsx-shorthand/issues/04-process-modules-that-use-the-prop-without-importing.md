# 04 — Process modules that use the prop without importing anything

**What to build:** A leaf component that only forwards the prop gets styled,
without its author having to add an import it does not otherwise need. This
holds for every bundler plugin, not only the one the report happened to test,
and it makes the plugins agree with the compiler, which already handles such a
module correctly when called directly.

The shared predicate keeps its import scan and gains a second condition: when
the prop name is configured, a module that mentions that name in a prop-like
position is handled too. The pattern is built from the resolved option value
and escaped, so a renamed prop is honoured and a name containing pattern
metacharacters cannot corrupt it. It matches the name followed by an equals
sign, a colon, a comma or a closing brace, with optional whitespace, which
covers the attribute form, the explicit property, the shorthand, and the
shorthand followed by further properties.

When the prop is disabled the second condition is skipped entirely and module
selection is exactly what it is today, so turning the feature off costs
nothing.

This is a text scan, not a parse. A mention inside a string or a comment
matches. That trade is deliberate: a false positive costs one compile that
produces unchanged output, while a false negative costs a silently unstyled
element. The import half of the predicate is not touched — tightening it would
drop modules that are processed today.

Behaviour is pinned twice: directly on the predicate, table-driven over the
cases that are awkward to reach through a plugin — a renamed prop, a name
needing escaping, the disabled option, and each prop-like position — and at the
plugin seam, by extending the existing test that asserts a module with no
import is skipped.

**Blocked by:** 01 — the shared predicate this condition is added to.

**Status:** done

- [x] A module using the prop with no import is handed to the compiler
- [x] A module using neither the prop nor an import is still skipped
- [x] With the prop disabled, a module using the prop is still skipped
- [x] A renamed prop is honoured, and a name containing pattern
      metacharacters is escaped rather than interpreted
- [x] The predicate is pinned by a table-driven unit test in the shared package
- [x] The behaviour is proven once at a plugin's transform seam, not only on
      the predicate
- [x] The other plugin packages get no new tests; they call the same function
- [x] Type checking, type-aware linting, linting, formatting and the
      JavaScript suites pass

**Note added while implementing:** the pattern also needs the name to start a
word. Without that, the default name matches through the JSX runtime import
that almost every already-compiled module carries — `import { jsx } from
"react/jsx-runtime"` matches on `jsx }` — which would hand nearly the whole
project to the compiler and lose the saving user story 10 asks for.
