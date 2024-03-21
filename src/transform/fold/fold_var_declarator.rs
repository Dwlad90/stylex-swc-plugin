use std::collections::{hash_map::Entry, HashMap};

use swc_core::{
    common::{comments::Comments, DUMMY_SP},
    ecma::{
        ast::{
            BindingIdent, Expr, Id, Ident, KeyValueProp, Lit, ObjectLit, Pat, Prop, PropName,
            PropOrSpread, VarDeclarator,
        },
        visit::FoldWith,
    },
};

use crate::{
    shared::enums::{
        ModuleCycle, NonNullProp, NonNullProps, StyleVarsToKeep, TopLevelExpression,
        TopLevelExpressionKind,
    },
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_var_declarator_impl(
        &mut self,
        mut var_declarator: VarDeclarator,
    ) -> VarDeclarator {
        // Get the declarations from the VarDecl struct
        // let var_declarator_id = var_declarator.clone().name.as_ident().unwrap().to_id();
        // let stylex_var_declarator = self.declaration.clone().unwrap();

        if self.cycle != ModuleCycle::Initializing
            && self.cycle != ModuleCycle::TransformEnter
            && self.cycle != ModuleCycle::TransformExit
        {
            if self.cycle == ModuleCycle::Cleaning {
                let mut vars_to_keep: HashMap<Id, NonNullProps> = HashMap::new();

                dbg!(&self.state.style_vars_to_keep);
                for StyleVarsToKeep(var_name, namespace_name, _) in
                    self.state.style_vars_to_keep.clone().into_iter()
                {
                    match vars_to_keep.entry(var_name.clone()) {
                        Entry::Occupied(mut entry) => {
                            if let NonNullProps::Vec(vec) = entry.get_mut() {
                                if let NonNullProp::Id(id) = &namespace_name {
                                    vec.push(id.clone());
                                }
                            }
                        }
                        Entry::Vacant(entry) => {
                            let value = match namespace_name {
                                NonNullProp::Id(namespace_name) => {
                                    NonNullProps::Vec(vec![namespace_name])
                                }
                                NonNullProp::True => NonNullProps::True,
                            };
                            entry.insert(value);
                        }
                    }
                }

                for (_, var_name) in self.state.style_vars.clone().into_iter() {
                    let var_decl = self.state.top_level_expressions.clone().into_iter().find(
                        |TopLevelExpression(_, expr)| {
                            var_name.init.clone().unwrap().eq(&Box::new(expr.clone()))
                        },
                    );

                    if let Option::Some(TopLevelExpression(kind, _)) = var_decl {
                        if TopLevelExpressionKind::Stmt == kind {
                            if let Some(object) = var_declarator.init.as_mut() {
                                if let Some(mut object) = object.as_object().cloned() {
                                    dbg!(&var_name.name, &vars_to_keep);

                                    let namespace_to_keep = match vars_to_keep
                                        .get(&var_name.name.as_ident().unwrap().to_id())
                                    {
                                        Some(e) => match e {
                                            NonNullProps::Vec(vec) => vec.clone(),
                                            NonNullProps::True => vec![],
                                        },
                                        None => vec![],
                                    };

                                    if !namespace_to_keep.is_empty() {
                                        let props = self.retain_object_props(
                                            &object,
                                            namespace_to_keep,
                                            var_name,
                                        );

                                        dbg!(&props);

                                        object.props = props;

                                        var_declarator.init =
                                            Option::Some(Box::new(Expr::Object(object)));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return var_declarator;
        }

        if &var_declarator.init.is_some() == &true {
            match &*var_declarator.init.clone().unwrap() {
                Expr::Call(call) => {
                    let declaration_tuple = self.process_declaration(&call);

                    match &declaration_tuple {
                        Some(declaration) => {
                            let (declaration, member) = declaration;

                            let stylex_imports = self.state.stylex_import_stringified();

                            if let Some(declaration_string) = stylex_imports
                                .into_iter()
                                .find(|item| item == &declaration.0.to_string())
                                .or_else(|| {
                                    self.state
                                        .stylex_create_import
                                        .clone()
                                        .into_iter()
                                        .find(|decl| decl.eq(&declaration))
                                        .and_then(|decl| Option::Some(decl.0.to_string()))
                                })
                            {
                                if member.as_str() == "create"
                                    || member.as_str() == declaration_string
                                {
                                    if self.cycle == ModuleCycle::Initializing {
                                        self.props_declaration = var_declarator
                                            .name
                                            .as_ident()
                                            .map(|ident| ident.to_id());
                                    } else {
                                        if self.state.options.runtime_injection.is_none() {
                                            var_declarator.name = Pat::Ident(BindingIdent {
                                                id: Ident {
                                                    span: DUMMY_SP,
                                                    optional: false,
                                                    sym: "_stylex$props".into(),
                                                },
                                                type_ann: None,
                                            })
                                        }
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }

        // Call the fold_children_with method on the VarDecl struct
        var_declarator.fold_children_with(self)
    }

    fn retain_object_props(
        &mut self,
        object: &ObjectLit,
        namespace_to_keep: Vec<Id>,
        var_name: VarDeclarator,
    ) -> Vec<PropOrSpread> {
        let props = object
            .props
            .clone()
            .into_iter()
            .filter_map(|mut object_prop| {
                assert!(object_prop.is_prop(), "Spread properties are not supported");

                let prop = object_prop.as_mut_prop().unwrap().as_mut();

                if let Some(KeyValueProp { key, .. }) = prop.as_key_value() {
                    let key_as_ident = match key {
                        PropName::Ident(ident) => Option::Some(ident),
                        // PropName::Str(str) => {
                        //     Option::Some(str.value.to_string())
                        // }
                        // PropName::Num(num) => {
                        //     Option::Some(num.value.to_string())
                        // }
                        // PropName::Computed(_) => Option::None,
                        // PropName::BigInt(big_int) => {
                        //     Option::Some(big_int.value.to_string())
                        // }
                        _ => None,
                    };

                    if let Some(key_as_string) = key_as_ident {
                        if namespace_to_keep.contains(&key_as_string.to_id()) {
                            let all_nulls_to_keep = self
                                .state
                                .style_vars_to_keep
                                .clone()
                                .into_iter()
                                .filter(|StyleVarsToKeep(v, namespace_name, _)| {
                                    var_name.name.clone().as_ident().unwrap().to_id().eq(v)
                                        && namespace_name
                                            .eq(&NonNullProp::Id(key_as_ident.unwrap().to_id()))
                                })
                                .map(|StyleVarsToKeep(_, _, non_null_props)| non_null_props)
                                .collect::<Vec<NonNullProps>>();

                            if !all_nulls_to_keep.contains(&NonNullProps::True) {
                                let nulls_to_keep = all_nulls_to_keep
                                    .into_iter()
                                    .filter_map(|item| match item {
                                        NonNullProps::Vec(vec) => Option::Some(vec),
                                        NonNullProps::True => Option::None,
                                    })
                                    .flatten()
                                    .collect::<Vec<Id>>();

                                if let Some(style_object) = prop
                                    .as_mut_key_value()
                                    .expect("Prop not a key value")
                                    .value
                                    .as_mut_object()
                                {
                                    retain_style_props(style_object, nulls_to_keep);
                                }
                            }

                            return Some(object_prop);
                        }
                    }
                }

                Option::None
            })
            .collect::<Vec<PropOrSpread>>();

        props
    }
}

fn retain_style_props(style_object: &mut ObjectLit, nulls_to_keep: Vec<Id>) {
    style_object.props.retain(|prop| {
        assert!(prop.is_prop(), "Spread properties are not supported");

        let style_prop = prop.as_prop().unwrap().as_ref();

        match style_prop {
            Prop::KeyValue(key_value) => {
                let value = key_value.value.clone();

                if let Some(Lit::Null(_)) = value.as_lit() {
                    let style_key_as_ident = match key_value.key.clone() {
                        PropName::Ident(ident) => Option::Some(ident),
                        // PropName::Str(str) => {
                        //     Option::Some(str.value.to_string())
                        // }
                        // PropName::Num(num) => {
                        //     Option::Some(num.value.to_string())
                        // }
                        // PropName::Computed(_) => Option::None,
                        // PropName::BigInt(big_int) => {
                        //     Option::Some(big_int.value.to_string())
                        // }
                        _ => todo!("Not implemented yet"),
                    };

                    style_key_as_ident.map_or(false, |style_key_as_string| {
                        nulls_to_keep.contains(&style_key_as_string.to_id())
                    })
                } else {
                    true
                }
            }
            _ => true,
        }
    });
}
