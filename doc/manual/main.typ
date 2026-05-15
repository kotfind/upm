#import "../common/template.typ": template
#import "../common/cfg.typ": cfg

#show: body => template(
    cfg: cfg,
    doc_name: "Руководство оператора",
    doc_code: "34",
    body,
)

#include "body.typ"
