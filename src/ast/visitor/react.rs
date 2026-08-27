use swc_common::SourceMap;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitMut, VisitMutWith, VisitWith};

use crate::ast::visitor::resolve_ts_type;
use crate::ast::{PropDetail, ReactComponentAnalysis, StateDetail, TailwindMode};
use crate::ollama::OllamaUrai;

pub struct ReactJsxPruner<'a> {
    pub mode: TailwindMode,
    pub ollama: Option<&'a OllamaUrai>,
    pub tailwind_threshold: usize,
}

impl<'a> VisitMut for ReactJsxPruner<'a> {
    fn visit_mut_jsx_opening_element(&mut self, el: &mut JSXOpeningElement) {
        let mut pruned_attrs = Vec::new();

        for attr in el.attrs.drain(..) {
            match attr {
                JSXAttrOrSpread::JSXAttr(jsx_attr) => {
                    let is_classname = match &jsx_attr.name {
                        JSXAttrName::Ident(ident) => {
                            ident.sym == "className" || ident.sym == "style"
                        }
                        _ => false,
                    };

                    if is_classname {
                        match self.mode {
                            TailwindMode::Remove => {
                                // If value is static string literal, strip it!
                                // If value is JSXExprContainer (dynamic expression like clsx or ternary), keep it.
                                if let Some(JSXAttrValue::JSXExprContainer(expr)) = jsx_attr.value {
                                    pruned_attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                                        span: jsx_attr.span,
                                        name: jsx_attr.name,
                                        value: Some(JSXAttrValue::JSXExprContainer(expr)),
                                    }));
                                }
                            }
                            TailwindMode::Summarize => {
                                if let Some(JSXAttrValue::Str(ref s)) = jsx_attr.value {
                                    let raw_str = s.value.as_str().unwrap_or_default().to_string();
                                    let summary = if raw_str.len() > self.tailwind_threshold {
                                        if let Some(ollama) = self.ollama {
                                            ollama
                                                .summarize_tailwind_classes(&raw_str)
                                                .unwrap_or_else(|_| {
                                                    "Styled component layout".to_string()
                                                })
                                        } else {
                                            format!(
                                                "/* Style: {} classes */",
                                                raw_str.split_whitespace().count()
                                            )
                                        }
                                    } else {
                                        format!(
                                            "/* Style: {} classes */",
                                            raw_str.split_whitespace().count()
                                        )
                                    };

                                    pruned_attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                                        span: jsx_attr.span,
                                        name: jsx_attr.name,
                                        value: Some(JSXAttrValue::Str(Str {
                                            span: s.span,
                                            value: format!("/* UI: {} */", summary).into(),
                                            raw: None,
                                        })),
                                    }));
                                } else {
                                    pruned_attrs.push(JSXAttrOrSpread::JSXAttr(jsx_attr));
                                }
                            }
                            TailwindMode::Preserve => {
                                pruned_attrs.push(JSXAttrOrSpread::JSXAttr(jsx_attr));
                            }
                            TailwindMode::RemoveAggr => {
                                if let Some(JSXAttrValue::JSXExprContainer(expr)) = jsx_attr.value {
                                    pruned_attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                                        span: jsx_attr.span,
                                        name: jsx_attr.name,
                                        value: Some(JSXAttrValue::JSXExprContainer(expr)),
                                    }));
                                }
                            }
                        }
                    } else {
                        pruned_attrs.push(JSXAttrOrSpread::JSXAttr(jsx_attr));
                    }
                }
                JSXAttrOrSpread::SpreadElement(spread) => {
                    pruned_attrs.push(JSXAttrOrSpread::SpreadElement(spread));
                }
            }
        }

        el.attrs = pruned_attrs;
        el.visit_mut_children_with(self);
    }
}

pub struct ReactComponentAnalyzer<'a> {
    pub cm: &'a SourceMap,
    pub components: Vec<ReactComponentAnalysis>,
}

impl<'a> ReactComponentAnalyzer<'a> {
    pub fn new(cm: &'a SourceMap) -> Self {
        Self {
            cm,
            components: Vec::new(),
        }
    }
}

impl<'a> Visit for ReactComponentAnalyzer<'a> {
    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        let name = fn_decl.ident.sym.to_string();
        if is_component_name(&name) {
            let analysis = analyze_function_body(&name, &fn_decl.function);
            self.components.push(analysis);
        }
        fn_decl.visit_children_with(self);
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) {
        for decl in &var_decl.decls {
            if let Pat::Ident(ident) = &decl.name {
                let name = ident.id.sym.to_string();
                if is_component_name(&name) {
                    if let Some(init) = &decl.init {
                        if let Expr::Arrow(arrow) = &**init {
                            let analysis = analyze_arrow_body(&name, arrow);
                            self.components.push(analysis);
                        } else if let Expr::Fn(fn_expr) = &**init {
                            let analysis = analyze_function_body(&name, &fn_expr.function);
                            self.components.push(analysis);
                        }
                    }
                }
            }
        }
        var_decl.visit_children_with(self);
    }
}

fn is_component_name(name: &str) -> bool {
    name.chars().next().map_or(false, |c| c.is_uppercase())
}

fn analyze_function_body(name: &str, func: &Function) -> ReactComponentAnalysis {
    let mut props = Vec::new();
    let mut state_vars = Vec::new();
    let mut effect_count = 0;
    let mut hooks_used = Vec::new();
    let mut event_handlers = Vec::new();
    let mut rendered_elements = Vec::new();

    // Analyze parameters for Props
    if let Some(first_param) = func.params.first() {
        match &first_param.pat {
            Pat::Ident(ident) => {
                let prop_name = ident.id.sym.to_string();
                let prop_type = ident
                    .type_ann
                    .as_ref()
                    .map(|ta| resolve_ts_type(&ta.type_ann))
                    .unwrap_or_else(|| "any".to_string());

                props.push(PropDetail {
                    name: prop_name,
                    prop_type,
                    is_optional: ident.id.optional,
                });
            }
            Pat::Object(obj_pat) => {
                for prop in &obj_pat.props {
                    if let ObjectPatProp::KeyValue(kv) = prop {
                        if let PropName::Ident(id) = &kv.key {
                            props.push(PropDetail {
                                name: id.sym.to_string(),
                                prop_type: "any".to_string(),
                                is_optional: false,
                            });
                        }
                    } else if let ObjectPatProp::Assign(assign) = prop {
                        props.push(PropDetail {
                            name: assign.key.sym.to_string(),
                            prop_type: "any".to_string(),
                            is_optional: assign.value.is_some(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(body) = &func.body {
        let mut inspector = ComponentBodyInspector::default();
        body.visit_with(&mut inspector);
        state_vars = inspector.state_vars;
        effect_count = inspector.effect_count;
        hooks_used = inspector.hooks_used;
        event_handlers = inspector.event_handlers;
        rendered_elements = inspector.rendered_elements;
    }

    let detailed_explanation = generate_component_explanation(
        name,
        &props,
        &state_vars,
        effect_count,
        &hooks_used,
        &event_handlers,
    );

    ReactComponentAnalysis {
        component_name: name.to_string(),
        is_export_default: false,
        props,
        state_vars,
        effect_count,
        hooks_used,
        event_handlers,
        rendered_elements,
        pruned_jsx_code: String::new(),
        detailed_explanation,
    }
}

fn analyze_arrow_body(name: &str, arrow: &ArrowExpr) -> ReactComponentAnalysis {
    let mut props = Vec::new();

    if let Some(first_param) = arrow.params.first() {
        match first_param {
            Pat::Ident(ident) => {
                let prop_name = ident.id.sym.to_string();
                let prop_type = ident
                    .type_ann
                    .as_ref()
                    .map(|ta| resolve_ts_type(&ta.type_ann))
                    .unwrap_or_else(|| "any".to_string());

                props.push(PropDetail {
                    name: prop_name,
                    prop_type,
                    is_optional: ident.id.optional,
                });
            }
            Pat::Object(obj_pat) => {
                for prop in &obj_pat.props {
                    if let ObjectPatProp::Assign(assign) = prop {
                        props.push(PropDetail {
                            name: assign.key.sym.to_string(),
                            prop_type: "any".to_string(),
                            is_optional: assign.value.is_some(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    let mut inspector = ComponentBodyInspector::default();
    match &*arrow.body {
        BlockStmtOrExpr::BlockStmt(block) => {
            block.visit_with(&mut inspector);
        }
        BlockStmtOrExpr::Expr(expr) => {
            expr.visit_with(&mut inspector);
        }
    }

    let state_vars = inspector.state_vars;
    let effect_count = inspector.effect_count;
    let hooks_used = inspector.hooks_used;
    let event_handlers = inspector.event_handlers;
    let rendered_elements = inspector.rendered_elements;

    let detailed_explanation = generate_component_explanation(
        name,
        &props,
        &state_vars,
        effect_count,
        &hooks_used,
        &event_handlers,
    );

    ReactComponentAnalysis {
        component_name: name.to_string(),
        is_export_default: false,
        props,
        state_vars,
        effect_count,
        hooks_used,
        event_handlers,
        rendered_elements,
        pruned_jsx_code: String::new(),
        detailed_explanation,
    }
}

#[derive(Default)]
struct ComponentBodyInspector {
    state_vars: Vec<StateDetail>,
    effect_count: usize,
    hooks_used: Vec<String>,
    event_handlers: Vec<String>,
    rendered_elements: Vec<String>,
}

impl Visit for ComponentBodyInspector {
    fn visit_call_expr(&mut self, call: &CallExpr) {
        if let Callee::Expr(callee_expr) = &call.callee {
            if let Expr::Ident(ident) = &**callee_expr {
                let name = ident.sym.as_str();
                if name.starts_with("use") {
                    if !self.hooks_used.contains(&name.to_string()) {
                        self.hooks_used.push(name.to_string());
                    }
                    if name == "useEffect" || name == "useLayoutEffect" {
                        self.effect_count += 1;
                    }
                }
            }
        }
        call.visit_children_with(self);
    }

    fn visit_var_decl(&mut self, decl: &VarDecl) {
        for declarator in &decl.decls {
            if let Pat::Array(arr) = &declarator.name {
                if let Some(init) = &declarator.init {
                    if let Expr::Call(call) = &**init {
                        if let Callee::Expr(callee_expr) = &call.callee {
                            if let Expr::Ident(ident) = &**callee_expr {
                                if ident.sym == "useState" {
                                    let state_name = arr
                                        .elems
                                        .first()
                                        .and_then(|e| e.as_ref())
                                        .and_then(|p| match p {
                                            Pat::Ident(id) => Some(id.id.sym.to_string()),
                                            _ => None,
                                        })
                                        .unwrap_or_else(|| "state".to_string());

                                    let setter_name = arr
                                        .elems
                                        .get(1)
                                        .and_then(|e| e.as_ref())
                                        .and_then(|p| match p {
                                            Pat::Ident(id) => Some(id.id.sym.to_string()),
                                            _ => None,
                                        })
                                        .unwrap_or_else(|| "setState".to_string());

                                    self.state_vars.push(StateDetail {
                                        state_name,
                                        setter_name,
                                        initial_value: None,
                                        state_type: "any".to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        decl.visit_children_with(self);
    }

    fn visit_jsx_element(&mut self, el: &JSXElement) {
        let tag_name = match &el.opening.name {
            JSXElementName::Ident(ident) => ident.sym.to_string(),
            JSXElementName::JSXMemberExpr(mem) => mem.prop.sym.to_string(),
            _ => "Fragment".to_string(),
        };

        if !self.rendered_elements.contains(&tag_name) {
            self.rendered_elements.push(tag_name);
        }
        el.visit_children_with(self);
    }
}

fn generate_component_explanation(
    name: &str,
    props: &[PropDetail],
    state_vars: &[StateDetail],
    effect_count: usize,
    hooks_used: &[String],
    event_handlers: &[String],
) -> String {
    let mut exp = format!("### React Component Breakdown: `<{}>` \n\n", name);

    if props.is_empty() {
        exp.push_str("- **Props**: Receives no explicit props (or uses `children` only).\n");
    } else {
        exp.push_str("- **Props**:\n");
        for p in props {
            exp.push_str(&format!(
                "  - `{}` (type: `{}`){}\n",
                p.name,
                p.prop_type,
                if p.is_optional { " [optional]" } else { "" }
            ));
        }
    }

    if state_vars.is_empty() {
        exp.push_str("- **State**: Stateless component.\n");
    } else {
        exp.push_str("- **State Management**:\n");
        for s in state_vars {
            exp.push_str(&format!(
                "  - Manages state `{}` via setter `{}`.\n",
                s.state_name, s.setter_name
            ));
        }
    }

    if !hooks_used.is_empty() {
        exp.push_str(&format!(
            "- **Hooks**: Uses `{}` (Total Side-Effects: {}).\n",
            hooks_used.join(", "),
            effect_count
        ));
    }

    if !event_handlers.is_empty() {
        exp.push_str(&format!(
            "- **Event Handlers**: Handlers attached: `{}`.\n",
            event_handlers.join(", ")
        ));
    }

    exp
}
