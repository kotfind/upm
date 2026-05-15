#import "../common/template.typ": template
#import "../common/cfg.typ": cfg

#show: body => template(
    cfg: cfg,
    doc_name: "Техническое задание",
    doc_code: "ТЗ",
    body,
)

#include "body.typ"
