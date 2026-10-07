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
        grid(
          columns: (1fr, auto),
          [CC BY 4.0],
          counter(page).display("1"),
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

  show raw: set text(font: mono-font, size: 9pt)
  show raw.where(block: true): it => block(
    width: 100%,
    fill: luma(245),
    inset: 8pt,
    radius: 2pt,
    it,
  )

  set table(stroke: 0.5pt + luma(120), inset: 5pt)
  show table: set text(size: 10.5pt)
  show figure.where(kind: table): set figure.caption(position: top)

  show link: set text(fill: rgb("#1a4d8f"))

  body
}
