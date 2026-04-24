#import "/common/template.typ": template
#import "/common/cfg.typ": cfg

#show: body => template(cfg: cfg, doc_name: "Техническое задание", body)

#include "body.typ"
