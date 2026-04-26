#import "../common/template.typ": template
#import "../common/cfg.typ": cfg

#show: body => template(
    cfg: cfg,
    doc_name: "Программа и методика испытаний",
    doc_code: "51",
    body,
)

#include "body.typ"
