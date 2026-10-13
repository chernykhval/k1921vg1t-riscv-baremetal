// Page layout and typography of the report.
// Fonts are the ones embedded in typst (build with --ignore-system-fonts),
// so the PDF is identical on any machine.

#let body-font = "Libertinus Serif"
#let mono-font = "DejaVu Sans Mono"

// Visible marker for unfinished places; the final build must contain none.
#let todo(body) = text(fill: rgb("#c00000"), weight: "bold")[[TODO: #body]]

#let report(title: "", author: "", keywords: (), body) = {
  set document(title: title, author: author, keywords: keywords)

  set page(
    paper: "a4",
    margin: (left: 30mm, right: 15mm, top: 20mm, bottom: 20mm),
    footer: context {
      if counter(page).get().first() > 1 {
        set text(size: 9pt)
        // Page number centred (GOST 7.32), licence mark on the left.
        grid(
          columns: (1fr, auto, 1fr),
          align(left)[CC BY 4.0],
          counter(page).display("1"),
          [],
        )
      }
    },
  )

  set text(font: body-font, size: 13pt, lang: "ru", region: "ru", hyphenate: true)
  set par(justify: true, leading: 0.75em, spacing: 0.9em, first-line-indent: (amount: 1.25cm, all: true))

  set heading(numbering: "1.1.")
  show heading: set block(above: 1.4em, below: 0.9em)
  show heading.where(level: 1): it => {
    pagebreak(weak: true)
    set text(size: 16pt)
    it
  }
  show heading.where(level: 2): set text(size: 14pt)

  show raw: set text(font: mono-font, size: 1.05em) // typst already scales raw text by 0.8
  show raw.where(block: true): set text(size: 9pt)
  show raw.where(block: true): it => block(
    width: 100%,
    fill: luma(245),
    inset: 8pt,
    radius: 2pt,
    it,
  )

  set table(stroke: 0.5pt + luma(120), inset: 5pt)
  show table: set text(size: 10.5pt)
  show table: set par(justify: false)
  show figure.where(kind: table): set figure.caption(position: top)
  // GOST-style captions: "Таблица 1 — …", "Рисунок 1 — …".
  show figure.where(kind: image): set figure(supplement: [Рисунок])
  set figure.caption(separator: [ — ])
  show figure: set block(above: 1.4em, below: 1.4em)
  // Long tables may continue on the next page instead of leaving half a page empty.
  show figure.where(kind: table): set block(breakable: true)
  // "Таблица N" / "Рисунок N" in bold so the caption does not merge with the body text.
  // A caption above its table is sticky, so it never stays alone at the bottom of a page.
  show figure.caption: set par(justify: false)
  show figure.caption: it => context block(sticky: it.position == top, width: 100%)[
    *#it.supplement~#it.counter.display(it.numbering)*#it.separator#it.body
  ]

  show link: set text(fill: rgb("#1a4d8f"), hyphenate: false)
  // Cyrillic words inside formulas (units like "МГц") in the body font, not the math font.
  show math.equation: it => {
    show regex("[А-Яа-яЁё]+"): set text(font: body-font)
    it
  }

  body
}
