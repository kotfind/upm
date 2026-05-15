#import "prelude.typ": *

#set text(lang: "ru")

#set table(
    align: left,
    inset: 9mm,
)

#show: university-theme.with(
    aspect-ratio: "16-9",
    align: center + horizon,
    config-info(
        title: [USB Менеджер Паролей],
        subtitle: [Курсовая работа],
        author: [Чубий Савва Андреевич],
    ),
)

#title-slide(
    title: [Курсовая работа],
    subtitle: [USB Менеджер Паролей],
    authors: grid(
        columns: (auto, 15cm),
        align: (right, left),
        gutter: 3mm,
        [Студент:], [Чубий Савва Андреевич],
        [], d[БПИ 232],
        [Научный руководитель:], [Макаров Сергей Львович],
        [], d[Доцент департамента программной инженерии],
    ),
)

#include "slides.typ"
