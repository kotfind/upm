#import "../common/template.typ": template
#import "../common/cfg.typ": cfg

#show: body => template(
    cfg: cfg,
    doc_name: "Программа и методика испытаний",
    body,
)

#include "body.typ"
