// Research report with a practicum (Russian).
// Build from the repository root:
//   typst compile --root . --ignore-system-fonts docs/report/main.typ docs/k1921vg1t-rust-baremetal-report.pdf
#import "lib/template.typ": report
#import "lib/title.typ": title-page

#let title = "Bare-metal разработка для микроконтроллера К1921ВГ1Т, включая оценку применимости языка Rust (no_std) и подготовку открытой документации"
#let author = "Черных В.А."
#let repo = "https://github.com/chernykhval/k1921vg1t-riscv-baremetal"

#show: report.with(
  title: title,
  author: author,
  keywords: ("К1921ВГ1Т", "RISC-V", "Rust", "no_std", "PAC", "SVD", "bare-metal"),
)

#title-page(title: title, author: author, repo: repo)

#pagebreak()
#outline(title: "Содержание", depth: 2)

#include "chapters/00-terms.typ"
#include "chapters/01-abstract.typ"
#include "chapters/02-relevance-novelty.typ"
#include "chapters/03-environment.typ"
#include "chapters/04-pac.typ"
#include "chapters/05-peripherals.typ"
#include "chapters/06-rust-assessment.typ"
#include "chapters/07-defects.typ"
#include "chapters/08-practicum.typ"
#include "chapters/09-conclusion.typ"

#bibliography("refs.yml", title: "Список источников", style: "gost-r-705-2008-numeric")

// Appendices: unnumbered level-1 headings whose text starts with "Приложение А." etc., so the
// same text appears in the heading and in the table of contents (GOST 7.32) without widening
// the number column of the contents.
#set heading(numbering: none)
#show heading.where(level: 1): it => {
  pagebreak(weak: true)
  set text(size: 16pt)
  block(above: 1.4em, below: 0.9em, it.body)
}
#include "appendix/a-svd-fixes.typ"
#include "appendix/b-jtag-wiring.typ"
