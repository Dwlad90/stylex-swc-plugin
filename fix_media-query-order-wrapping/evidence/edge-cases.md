# Edge cases, one recorded run each

Every expectation in the media-query edge-case tests, beside the run it was
taken from. Reproduced by `edge-cases.cjs` in this directory, which compiles
each subject through both compilers and compares the whole emitted rule list.

- reference implementation: `@stylexjs/babel-plugin` 0.19.0
- this compiler: `@stylexswc/rs-compiler` from `dist/`
- fifteen subjects, fifteen agreements

The `babel` line is the reference implementation's emitted rules in order; the
two compilers agreed on every one, so it is also this compiler's.

## vendor prefixed feature

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (-webkit-min-device-pixel-ratio: 2) and (not (max-width: 50px)){.x4b3nli.x4b3nli{color:red}}
@media (max-width: 50px){.x1ggxjco.x1ggxjco{color:blue}}
```

## moz prefixed feature

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (-moz-device-pixel-ratio: 2) and (not (max-width: 50px)){.x1vdho0q.x1vdho0q{color:red}}
@media (max-width: 50px){.x1ggxjco.x1ggxjco{color:blue}}
```

## prefixed beside a width ladder

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (-webkit-min-device-pixel-ratio: 2) and (min-width: 200px) and (not (min-width: 100px)){.x87pisy.x87pisy{color:red}}
@media (min-width: 100px){.x18tmubq.x18tmubq{color:blue}}
```

## emoji in a feature name

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (😀: 1) and (not (max-width: 50px)){.xv1rb8b.xv1rb8b{color:red}}
@media (max-width: 50px){.x1ggxjco.x1ggxjco{color:blue}}
```

## combining marks in a feature name

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (mín-width: 100px) and (not (max-width: 50px)){.x15sb3sw.x15sb3sw{color:red}}
@media (max-width: 50px){.x1ggxjco.x1ggxjco{color:blue}}
```

## escaped at sign

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (min-width: 100px) and (@foo: 1) and (not (max-width: 50px)){.x8k9vzd.x8k9vzd{color:red}}
@media (max-width: 50px){.x1ggxjco.x1ggxjco{color:blue}}
```

## trailing whitespace in the key

Verdict: **agree**

```text
both refused the declaration
```

## bare @media

Verdict: **agree**

```text
both refused the declaration
```

## only a default

Verdict: **agree**

```text
.x1mqxbix{color:black}
```

## a media key with no default

Verdict: **agree**

```text
@media not all{.x1jqaanj.x1jqaanj{color:red}}
@media (min-width: 100px){.x18tmubq.x18tmubq{color:blue}}
```

## comma separated disjuncts

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media not all, (max-width: 99.99px){.xr8driy.xr8driy{color:red}}
@media (min-width: 100px){.x18tmubq.x18tmubq{color:blue}}
```

## media type beside a width

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (screen) and (min-width: 200px) and (not (min-width: 100px)){.xmrqklk.xmrqklk{color:red}}
@media (min-width: 100px){.x18tmubq.x18tmubq{color:blue}}
```

## huge length

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (min-width: 1e+308px){.x1nk4rw.x1nk4rw{color:red}}
@media (max-width: 50px){.x1ggxjco.x1ggxjco{color:blue}}
```

## nan-ish length

Verdict: **agree**

```text
.x1mqxbix{color:black}
@media (min-width: 50.01px){.xta8mtt.xta8mtt{color:red}}
@media (max-width: 50px){.x1ggxjco.x1ggxjco{color:blue}}
```

## media keys eight levels deep

Verdict: **agree**

```text
both refused the declaration
```

