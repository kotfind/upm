#import "../common/template.typ": template
#import "../common/cfg.typ": cfg

#show: body => template(
    cfg: cfg,
    doc_name: "Пояснительная записка",
    doc_code: "81",
    body,
)

#include "body.typ"
