#import "../common/template.typ": template
#import "../common/cfg.typ": cfg

#show: body => template(
    cfg: cfg,
    doc_name: "Текст программы",
    doc_code: "12",
    body,
)

#let href(url) = box(underline(link(url)))

= ТЕКСТ ПРОГРАММЫ

Исходный код программы разделен на несколько GitHub-репозиториев:
- #href("https://github.com/kotfind/w25") --- драйвер для чипа памяти.
- #href("https://github.com/kotfind/rekv") --- реализация простой реляционной
    базы данных, обертка над библиотекой `ekv`.
- #href("https://github.com/kotfind/upm") --- основной проект.
    *Обратите внимание, что в репозитории содержится несколько веток.*
