// Title page.
#let title-page(title: "", author: "", repo: "") = {
  set par(first-line-indent: 0pt, justify: false)
  set text(hyphenate: false)
  set align(center)

  text(size: 12pt)[
    Российский Альянс RISC-V \
    Конкурс учебно-методических и исследовательских работ
  ]

  v(1fr)

  text(size: 12pt)[Исследовательская работа с учебно-методическим практикумом]
  v(0.8em)
  text(size: 18pt, weight: "bold", title)

  v(1fr)

  set align(left)
  stack(
    spacing: 0.7em,
    [Автор: #author, ННГУ им. Н.И. Лобачевского, группа 3824Б1ПР2],
    [Материалы: #link(repo)],
  )

  v(1.5em)
  block(
    width: 100%,
    stroke: 0.5pt + luma(120),
    inset: 8pt,
    text(size: 10pt)[
      // The author string already ends with a dot ("Черных В.А.").
      © 2026 #author Материалы распространяются на условиях лицензии
      Creative Commons «С~указанием авторства» версии 4.0 Международная (CC BY 4.0): \
      #link("https://creativecommons.org/licenses/by/4.0/legalcode.ru")
    ],
  )

  v(1fr)
  align(center)[2026]
}
