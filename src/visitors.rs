use syn::{visit::Visit, ItemFn, ItemMod, ItemStruct, ItemEnum, ItemTrait, ItemConst, ItemStatic, ItemType, ItemImpl, Visibility};

pub struct TocVisitor<'a> {
    pub items: &'a mut Vec<String>,
    pub current_mod: Vec<String>,
}

impl<'a> Visit<'_> for TocVisitor<'a> {
    fn visit_item_fn(&mut self, node: &ItemFn) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("pub fn {}{}", mod_path, node.sig.ident));
        }
    }
    
    fn visit_item_struct(&mut self, node: &ItemStruct) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("pub struct {}{}", mod_path, node.ident));
        }
    }
    
    fn visit_item_enum(&mut self, node: &ItemEnum) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("pub enum {}{}", mod_path, node.ident));
        }
    }
    
    fn visit_item_trait(&mut self, node: &ItemTrait) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("pub trait {}{}", mod_path, node.ident));
        }
    }
    
    fn visit_item_mod(&mut self, node: &ItemMod) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("pub mod {}{}", mod_path, node.ident));
        }
        
        if let Some((_, items)) = &node.content {
            self.current_mod.push(node.ident.to_string());
            for item in items {
                self.visit_item(item);
            }
            self.current_mod.pop();
        }
    }
    
    fn visit_item_const(&mut self, node: &ItemConst) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("pub const {}{}", mod_path, node.ident));
        }
    }
    
    fn visit_item_static(&mut self, node: &ItemStatic) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("pub static {}{}", mod_path, node.ident));
        }
    }
    
    fn visit_item_type(&mut self, node: &ItemType) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("pub type {}{}", mod_path, node.ident));
        }
    }
    
    fn visit_item_impl(&mut self, node: &syn::ItemImpl) {
        // 実装対象の型名を取得
        let impl_type = match &*node.self_ty {
            syn::Type::Path(type_path) => {
                type_path.path.segments.iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::")
            }
            _ => "Unknown".to_string(),
        };
        
        let mod_path = if self.current_mod.is_empty() {
            String::new()
        } else {
            format!("{}::", self.current_mod.join("::"))
        };
        
        // トレイト実装かどうか
        if let Some((_, trait_path, _)) = &node.trait_ {
            let trait_name = trait_path.segments.iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            
            self.items.push(format!("impl {} for {}{}", trait_name, mod_path, impl_type));
        } else {
            self.items.push(format!("impl {}{}", mod_path, impl_type));
        }
    }
}

pub struct SummaryVisitor<'a> {
    pub public_count: &'a mut usize,
    pub types: &'a mut Vec<String>,
}

impl<'a> Visit<'_> for SummaryVisitor<'a> {
    fn visit_item_fn(&mut self, node: &ItemFn) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("functions".to_string());
        }
    }
    
    fn visit_item_struct(&mut self, node: &ItemStruct) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("structs".to_string());
        }
    }
    
    fn visit_item_enum(&mut self, node: &ItemEnum) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("enums".to_string());
        }
    }
    
    fn visit_item_trait(&mut self, node: &ItemTrait) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("traits".to_string());
        }
    }
    
    fn visit_item_const(&mut self, node: &ItemConst) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("constants".to_string());
        }
    }
    
    fn visit_item_static(&mut self, node: &ItemStatic) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("statics".to_string());
        }
    }
    
    fn visit_item_type(&mut self, node: &ItemType) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("type aliases".to_string());
        }
    }
    
    fn visit_item_impl(&mut self, _node: &syn::ItemImpl) {
        *self.public_count += 1;
        self.types.push("implementations".to_string());
    }
}

pub struct CompleteDocsVisitor<'a> {
    pub content: &'a mut String,
    pub current_mod: Vec<String>,
}

impl<'a> Visit<'_> for CompleteDocsVisitor<'a> {
    fn visit_item_fn(&mut self, node: &ItemFn) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            
            self.content.push_str(&format!("### {}{}\n\n", mod_path, node.sig.ident));
            
            // クリーンな関数シグネチャを作成
            self.content.push_str("```rust\n");
            
            let mut sig = String::new();
            
            // pub fn
            sig.push_str("pub fn ");
            
            // 関数名
            sig.push_str(&node.sig.ident.to_string());
            
            // ジェネリクス
            if !node.sig.generics.params.is_empty() {
                sig.push('<');
                let generics: Vec<String> = node.sig.generics.params.iter()
                    .map(|p| {
                        match p {
                            syn::GenericParam::Type(tp) => tp.ident.to_string(),
                            syn::GenericParam::Lifetime(lp) => lp.lifetime.to_string(),
                            syn::GenericParam::Const(cp) => format!("const {}: {}", cp.ident, extract_type_name(&cp.ty)),
                        }
                    })
                    .collect();
                sig.push_str(&generics.join(", "));
                sig.push('>');
            }
            
            // パラメータ
            sig.push('(');
            let params: Vec<String> = node.sig.inputs.iter()
                .map(|input| {
                    match input {
                        syn::FnArg::Receiver(recv) => {
                            let mut param = String::new();
                            if recv.reference.is_some() {
                                param.push('&');
                                if recv.mutability.is_some() {
                                    param.push_str("mut ");
                                }
                            }
                            param.push_str("self");
                            param
                        }
                        syn::FnArg::Typed(pat_type) => {
                            let param_name = extract_pattern_name(&pat_type.pat);
                            format!("{}: {}", 
                                param_name, 
                                extract_type_name(&pat_type.ty))
                        }
                    }
                })
                .collect();
            
            if params.len() > 1 || (params.len() == 1 && !params[0].starts_with("self")) {
                sig.push('\n');
                for (i, param) in params.iter().enumerate() {
                    sig.push_str("    ");
                    sig.push_str(param);
                    if i < params.len() - 1 {
                        sig.push(',');
                    }
                    sig.push('\n');
                }
            } else if !params.is_empty() {
                sig.push_str(&params.join(", "));
            }
            sig.push(')');
            
            // 戻り値型
            if let syn::ReturnType::Type(_, ty) = &node.sig.output {
                sig.push_str(" -> ");
                sig.push_str(&extract_type_name(ty));
            }
            
            // where句
            if let Some(where_clause) = &node.sig.generics.where_clause {
                if !where_clause.predicates.is_empty() {
                    sig.push_str("\nwhere\n    ");
                    sig.push_str(&extract_where_clause(where_clause));
                }
            }
            
            self.content.push_str(&sig);
            self.content.push_str("\n```\n\n");
            
            // docsコメントを抽出
            self.extract_docs_for_item(&node.attrs);
        }
    }
    
    fn visit_item_struct(&mut self, node: &ItemStruct) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            
            self.content.push_str(&format!("### {}{}\n\n", mod_path, node.ident));
            
            self.content.push_str("```rust\n");
            
            // derive属性を抽出
            let derives = extract_derives(&node.attrs);
            if !derives.is_empty() {
                self.content.push_str(&format!("#[derive({})]\n", derives.join(", ")));
            }
            
            // cfg属性を抽出
            let cfg_attrs = extract_cfg_attributes(&node.attrs);
            if !cfg_attrs.is_empty() {
                self.content.push_str(&format!("#[cfg({})]\n", cfg_attrs.join(", ")));
            }
            
            // 構造体定義のクリーンな表示
            let mut struct_def = format!("pub struct {}", node.ident);
            
            // ジェネリクスを追加
            if !node.generics.params.is_empty() {
                struct_def.push('<');
                let generics: Vec<String> = node.generics.params.iter()
                    .map(|p| {
                        match p {
                            syn::GenericParam::Type(tp) => tp.ident.to_string(),
                            syn::GenericParam::Lifetime(lp) => lp.lifetime.to_string(),
                            syn::GenericParam::Const(cp) => format!("const {}: {}", cp.ident, extract_type_name(&cp.ty)),
                        }
                    })
                    .collect();
                struct_def.push_str(&generics.join(", "));
                struct_def.push('>');
            }
            
            // フィールドを表示
            match &node.fields {
                syn::Fields::Named(fields) => {
                    struct_def.push_str(" {\n");
                    for field in &fields.named {
                        if let Some(ident) = &field.ident {
                            if matches!(field.vis, Visibility::Public(_)) {
                                struct_def.push_str(&format!("    pub {}: {},\n", ident, extract_type_name(&field.ty)));
                            }
                        }
                    }
                    struct_def.push('}');
                }
                syn::Fields::Unnamed(fields) => {
                    struct_def.push('(');
                    let field_types: Vec<String> = fields.unnamed
                        .iter()
                        .filter(|f| matches!(f.vis, Visibility::Public(_)))
                        .map(|f| format!("pub {}", extract_type_name(&f.ty)))
                        .collect();
                    struct_def.push_str(&field_types.join(", "));
                    struct_def.push_str(");");
                }
                syn::Fields::Unit => {
                    struct_def.push(';');
                }
            }
            
            self.content.push_str(&struct_def);
            self.content.push_str("\n```\n\n");
            
            self.extract_docs_for_item(&node.attrs);
        }
    }
    
    fn visit_item_enum(&mut self, node: &ItemEnum) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            
            self.content.push_str(&format!("### {}{}\n\n", mod_path, node.ident));
            
            self.content.push_str("```rust\n");
            
            // derive属性を抽出
            let derives = extract_derives(&node.attrs);
            if !derives.is_empty() {
                self.content.push_str(&format!("#[derive({})]\n", derives.join(", ")));
            }
            
            // cfg属性を抽出
            let cfg_attrs = extract_cfg_attributes(&node.attrs);
            if !cfg_attrs.is_empty() {
                self.content.push_str(&format!("#[cfg({})]\n", cfg_attrs.join(", ")));
            }
            
            // 列挙型定義のクリーンな表示
            let mut enum_def = format!("pub enum {}", node.ident);
            
            // ジェネリクスを追加
            if !node.generics.params.is_empty() {
                enum_def.push('<');
                let generics: Vec<String> = node.generics.params.iter()
                    .map(|p| {
                        match p {
                            syn::GenericParam::Type(tp) => tp.ident.to_string(),
                            syn::GenericParam::Lifetime(lp) => lp.lifetime.to_string(),
                            syn::GenericParam::Const(cp) => format!("const {}: {}", cp.ident, extract_type_name(&cp.ty)),
                        }
                    })
                    .collect();
                enum_def.push_str(&generics.join(", "));
                enum_def.push('>');
            }
            
            enum_def.push_str(" {\n");
            
            // バリアントを表示
            for variant in &node.variants {
                let mut variant_str = format!("    {}", variant.ident);
                
                // cfg属性を抽出
                let cfg_attrs = extract_cfg_attributes(&variant.attrs);
                if !cfg_attrs.is_empty() {
                    variant_str = format!("    #[cfg({})]\n    {}", cfg_attrs.join(", "), variant.ident);
                }
                
                // バリアントフィールドを表示
                match &variant.fields {
                    syn::Fields::Named(fields) => {
                        variant_str.push_str(" {");
                        let field_strs: Vec<String> = fields.named.iter()
                            .filter_map(|f| {
                                if let Some(ident) = &f.ident {
                                    if matches!(f.vis, syn::Visibility::Public(_)) {
                                        Some(format!(" pub {}: {}", ident, extract_type_name(&f.ty)))
                                    } else {
                                        Some(format!(" {}: {}", ident, extract_type_name(&f.ty)))
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if !field_strs.is_empty() {
                            variant_str.push_str(&field_strs.join(","));
                            variant_str.push(' ');
                        }
                        variant_str.push('}');
                    }
                    syn::Fields::Unnamed(fields) => {
                        variant_str.push('(');
                        let field_types: Vec<String> = fields.unnamed.iter()
                            .map(|f| extract_type_name(&f.ty))
                            .collect();
                        if field_types.is_empty() || field_types.iter().all(|t| t == "Unknown") {
                            variant_str.push_str("..");
                        } else {
                            variant_str.push_str(&field_types.join(", "));
                        }
                        variant_str.push(')');
                    }
                    syn::Fields::Unit => {
                        // Unit variant, no additional fields
                    }
                }
                
                enum_def.push_str(&format!("{},\n", variant_str));
            }
            
            enum_def.push('}');
            
            self.content.push_str(&enum_def);
            self.content.push_str("\n```\n\n");
            
            self.extract_docs_for_item(&node.attrs);
        }
    }
    
    fn visit_item_trait(&mut self, node: &ItemTrait) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            
            self.content.push_str(&format!("### {}{}\n\n", mod_path, node.ident));
            
            self.content.push_str("```rust\n");
            // トレイト定義のクリーンな表示
            let mut trait_signature = format!("pub trait {}", node.ident);
            
            // ジェネリクスを追加
            if !node.generics.params.is_empty() {
                trait_signature.push('<');
                let generics: Vec<String> = node.generics.params.iter()
                    .map(|p| {
                        match p {
                            syn::GenericParam::Type(tp) => tp.ident.to_string(),
                            syn::GenericParam::Lifetime(lp) => lp.lifetime.to_string(),
                            syn::GenericParam::Const(cp) => format!("const {}: {}", cp.ident, extract_type_name(&cp.ty)),
                        }
                    })
                    .collect();
                trait_signature.push_str(&generics.join(", "));
                trait_signature.push('>');
            }
            
            // Super traitsを追加
            if !node.supertraits.is_empty() {
                trait_signature.push_str(": ");
                let supertraits: Vec<String> = node.supertraits
                    .iter()
                    .map(|st| {
                        match st {
                            syn::TypeParamBound::Trait(trait_bound) => {
                                trait_bound.path.segments.iter()
                                    .map(|s| s.ident.to_string())
                                    .collect::<Vec<_>>()
                                    .join("::")
                            }
                            syn::TypeParamBound::Lifetime(lifetime) => lifetime.ident.to_string(),
                            _ => "Unknown".to_string(),
                        }
                    })
                    .collect();
                trait_signature.push_str(&supertraits.join(" + "));
            }
            
            trait_signature.push_str(" {");
            self.content.push_str(&trait_signature);
            
            // トレイトアイテムを完全なシグネチャで表示
            for item in &node.items {
                match item {
                    syn::TraitItem::Fn(method) => {
                        // 完全な関数シグネチャを構築
                        let mut sig = String::new();
                        
                        // fn
                        sig.push_str("fn ");
                        
                        // 関数名
                        sig.push_str(&method.sig.ident.to_string());
                        
                        // ジェネリクス
                        if !method.sig.generics.params.is_empty() {
                            sig.push('<');
                            let generics: Vec<String> = method.sig.generics.params.iter()
                                .map(|p| {
                                    match p {
                                        syn::GenericParam::Type(tp) => tp.ident.to_string(),
                                        syn::GenericParam::Lifetime(lp) => lp.lifetime.to_string(),
                                        syn::GenericParam::Const(cp) => format!("const {}: {}", cp.ident, extract_type_name(&cp.ty)),
                                    }
                                })
                                .collect();
                            sig.push_str(&generics.join(", "));
                            sig.push('>');
                        }
                        
                        // パラメータ
                        sig.push('(');
                        let params: Vec<String> = method.sig.inputs.iter()
                            .map(|input| {
                                match input {
                                    syn::FnArg::Receiver(recv) => {
                                        let mut param = String::new();
                                        if recv.reference.is_some() {
                                            param.push('&');
                                            if recv.mutability.is_some() {
                                                param.push_str("mut ");
                                            }
                                        }
                                        param.push_str("self");
                                        param
                                    }
                                    syn::FnArg::Typed(pat_type) => {
                                        let param_name = extract_pattern_name(&pat_type.pat);
                                        format!("{}: {}", param_name, extract_type_name(&pat_type.ty))
                                    }
                                }
                            })
                            .collect();
                        
                        if params.len() > 1 || (params.len() == 1 && !params[0].starts_with("self")) {
                            // 複数のパラメータがある場合は改行して表示
                            for (i, param) in params.iter().enumerate() {
                                if i > 0 {
                                    sig.push_str(",\n        ");
                                }
                                sig.push_str(param);
                            }
                        } else if !params.is_empty() {
                            sig.push_str(&params.join(", "));
                        }
                        sig.push(')');
                        
                        // 戻り値型
                        if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                            sig.push_str(" -> ");
                            sig.push_str(&extract_type_name(ty));
                        }
                        
                        self.content.push_str(&format!("\n    {};", sig));
                    }
                    syn::TraitItem::Type(ty) => {
                        self.content.push_str(&format!("\n    type {};", ty.ident));
                    }
                    syn::TraitItem::Const(c) => {
                        // クリーンなconst定義
                        let type_str = extract_type_name(&c.ty);
                        self.content.push_str(&format!("\n    const {}: {};", c.ident, type_str));
                    }
                    _ => {}
                }
            }
            
            self.content.push_str("\n}\n```\n\n");
            
            self.extract_docs_for_item(&node.attrs);
        }
    }
    
    fn visit_item_const(&mut self, node: &ItemConst) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            
            self.content.push_str(&format!("### {}{}\n\n", mod_path, node.ident));
            
            self.content.push_str("```rust\n");
            // クリーンなconst定義
            self.content.push_str(&format!(
                "pub const {}: {}",
                node.ident,
                extract_type_name(&node.ty)
            ));
            self.content.push_str("\n```\n\n");
            
            self.extract_docs_for_item(&node.attrs);
        }
    }
    
    fn visit_item_static(&mut self, node: &ItemStatic) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            
            self.content.push_str(&format!("### {}{}\n\n", mod_path, node.ident));
            
            self.content.push_str("```rust\n");
            // クリーンなstatic定義
            let mut static_def = String::new();
            static_def.push_str("pub static ");
            if matches!(node.mutability, syn::StaticMutability::Mut(_)) {
                static_def.push_str("mut ");
            }
            static_def.push_str(&format!(
                "{}: {}",
                node.ident,
                extract_type_name(&node.ty)
            ));
            self.content.push_str(&static_def);
            self.content.push_str("\n```\n\n");
            
            self.extract_docs_for_item(&node.attrs);
        }
    }
    
    fn visit_item_type(&mut self, node: &ItemType) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            
            self.content.push_str(&format!("### {}{}\n\n", mod_path, node.ident));
            
            self.content.push_str("```rust\n");
            // クリーンなtype alias定義
            let mut type_def = format!("pub type {}", node.ident);
            
            // ジェネリクスを追加
            if !node.generics.params.is_empty() {
                let generics: Vec<String> = node.generics.params.iter()
                    .map(|p| {
                        match p {
                            syn::GenericParam::Type(tp) => tp.ident.to_string(),
                            syn::GenericParam::Lifetime(lp) => lp.lifetime.to_string(),
                            syn::GenericParam::Const(cp) => format!("const {}: {}", cp.ident, extract_type_name(&cp.ty)),
                        }
                    })
                    .collect();
                type_def.push_str(&format!("<{}>", generics.join(", ")));
            }
            
            type_def.push_str(&format!(" = {}", extract_type_name(&node.ty)));
            
            // where句
            if let Some(where_clause) = &node.generics.where_clause {
                if !where_clause.predicates.is_empty() {
                    type_def.push_str("\nwhere\n    ");
                    type_def.push_str(&extract_where_clause(where_clause));
                }
            }
            
            self.content.push_str(&type_def);
            self.content.push_str("\n```\n\n");
            
            self.extract_docs_for_item(&node.attrs);
        }
    }
    
    fn visit_item_impl(&mut self, node: &ItemImpl) {
        // 実装対象の型名を取得
        let impl_type = match &*node.self_ty {
            syn::Type::Path(type_path) => {
                type_path.path.segments.iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::")
            }
            _ => "Unknown".to_string(),
        };
        
        let mod_path = if self.current_mod.is_empty() {
            String::new()
        } else {
            format!("{}::", self.current_mod.join("::"))
        };
        
        // トレイト実装かどうか
        if let Some((_, trait_path, _)) = &node.trait_ {
            let trait_name = trait_path.segments.iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            
            self.content.push_str(&format!("### impl {} for {}{}\n\n", trait_name, mod_path, impl_type));
        } else {
            self.content.push_str(&format!("### impl {}{}\n\n", mod_path, impl_type));
        }
        
        self.content.push_str("```rust\n");
        
        // impl シグネチャを構築
        let mut impl_sig = String::new();
        
        // ジェネリクス
        if !node.generics.params.is_empty() {
            impl_sig.push_str("impl<");
            let generics: Vec<String> = node.generics.params.iter()
                .map(|p| {
                    match p {
                        syn::GenericParam::Type(tp) => tp.ident.to_string(),
                        syn::GenericParam::Lifetime(lp) => lp.lifetime.to_string(),
                        syn::GenericParam::Const(cp) => format!("const {}: {}", cp.ident, extract_type_name(&cp.ty)),
                    }
                })
                .collect();
            impl_sig.push_str(&generics.join(", "));
            impl_sig.push_str("> ");
        } else {
            impl_sig.push_str("impl ");
        }
        
        // トレイト実装の場合
        if let Some((_, trait_path, _)) = &node.trait_ {
            let trait_name = trait_path.segments.iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            impl_sig.push_str(&format!("{} for ", trait_name));
        }
        
        impl_sig.push_str(&impl_type);
        impl_sig.push_str(" {");
        
        self.content.push_str(&impl_sig);
        
        // public メソッドを表示
        for item in &node.items {
            match item {
                syn::ImplItem::Fn(method) => {
                    if matches!(method.vis, Visibility::Public(_)) {
                        // 完全な関数シグネチャを構築
                        let mut sig = String::new();
                        
                        // pub fn
                        sig.push_str("pub fn ");
                        
                        // 関数名
                        sig.push_str(&method.sig.ident.to_string());
                        
                        // ジェネリクス
                        if !method.sig.generics.params.is_empty() {
                            sig.push('<');
                            let generics: Vec<String> = method.sig.generics.params.iter()
                                .map(|p| {
                                    match p {
                                        syn::GenericParam::Type(tp) => tp.ident.to_string(),
                                        syn::GenericParam::Lifetime(lp) => lp.lifetime.to_string(),
                                        syn::GenericParam::Const(cp) => format!("const {}: {}", cp.ident, extract_type_name(&cp.ty)),
                                    }
                                })
                                .collect();
                            sig.push_str(&generics.join(", "));
                            sig.push('>');
                        }
                        
                        // パラメータ
                        sig.push('(');
                        let params: Vec<String> = method.sig.inputs.iter()
                            .map(|input| {
                                match input {
                                    syn::FnArg::Receiver(recv) => {
                                        let mut param = String::new();
                                        if recv.reference.is_some() {
                                            param.push('&');
                                            if recv.mutability.is_some() {
                                                param.push_str("mut ");
                                            }
                                        }
                                        param.push_str("self");
                                        param
                                    }
                                    syn::FnArg::Typed(pat_type) => {
                                        let param_name = extract_pattern_name(&pat_type.pat);
                                        format!("{}: {}", param_name, extract_type_name(&pat_type.ty))
                                    }
                                }
                            })
                            .collect();
                        
                        if params.len() > 1 || (params.len() == 1 && !params[0].starts_with("self")) {
                            // 複数のパラメータがある場合は改行して表示
                            for (i, param) in params.iter().enumerate() {
                                if i > 0 {
                                    sig.push_str(",\n        ");
                                }
                                sig.push_str(param);
                            }
                        } else if !params.is_empty() {
                            sig.push_str(&params.join(", "));
                        }
                        sig.push(')');
                        
                        // 戻り値型
                        if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                            sig.push_str(" -> ");
                            sig.push_str(&extract_type_name(ty));
                        }
                        
                        self.content.push_str(&format!("\n    {};", sig));
                    }
                }
                syn::ImplItem::Const(const_item) => {
                    if matches!(const_item.vis, Visibility::Public(_)) {
                        self.content.push_str(&format!("\n    pub const {}: Type;", const_item.ident));
                    }
                }
                syn::ImplItem::Type(type_item) => {
                    if matches!(type_item.vis, Visibility::Public(_)) {
                        self.content.push_str(&format!("\n    pub type {};", type_item.ident));
                    }
                }
                _ => {}
            }
        }
        
        self.content.push_str("\n}\n```\n\n");
        
        // impl ブロックのdocsコメントがあれば抽出
        self.extract_docs_for_item(&node.attrs);
    }
}

fn extract_type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => {
            type_path.path.segments.iter()
                .map(|s| {
                    let mut segment = s.ident.to_string();
                    if !s.arguments.is_empty() {
                        match &s.arguments {
                            syn::PathArguments::AngleBracketed(args) => {
                                segment.push('<');
                                let arg_strs: Vec<String> = args.args.iter()
                                    .map(|arg| {
                                        match arg {
                                            syn::GenericArgument::Type(ty) => extract_type_name(ty),
                                            syn::GenericArgument::Lifetime(lt) => lt.ident.to_string(),
                                            syn::GenericArgument::Const(_) => "Const".to_string(),
                                            _ => "Unknown".to_string(),
                                        }
                                    })
                                    .collect();
                                segment.push_str(&arg_strs.join(", "));
                                segment.push('>');
                            }
                            syn::PathArguments::Parenthesized(_) => {
                                segment.push_str("(..)");
                            }
                            syn::PathArguments::None => {}
                        }
                    }
                    segment
                })
                .collect::<Vec<_>>()
                .join("::")
        }
        syn::Type::Reference(type_ref) => {
            let mut ref_str = "&".to_string();
            if type_ref.mutability.is_some() {
                ref_str.push_str("mut ");
            }
            ref_str.push_str(&extract_type_name(&type_ref.elem));
            ref_str
        }
        syn::Type::Slice(type_slice) => {
            format!("[{}]", extract_type_name(&type_slice.elem))
        }
        syn::Type::Array(type_array) => {
            format!("[{}; N]", extract_type_name(&type_array.elem))
        }
        syn::Type::Ptr(type_ptr) => {
            if type_ptr.mutability.is_some() {
                format!("*mut {}", extract_type_name(&type_ptr.elem))
            } else {
                format!("*const {}", extract_type_name(&type_ptr.elem))
            }
        }
        syn::Type::Tuple(type_tuple) => {
            let elem_strs: Vec<String> = type_tuple.elems.iter()
                .map(extract_type_name)
                .collect();
            format!("({})", elem_strs.join(", "))
        }
        syn::Type::ImplTrait(_) => "impl Trait".to_string(),
        syn::Type::TraitObject(_) => "dyn Trait".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn extract_pattern_name(pat: &syn::Pat) -> String {
    match pat {
        syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
        syn::Pat::Reference(pat_ref) => extract_pattern_name(&pat_ref.pat),
        syn::Pat::Type(pat_type) => extract_pattern_name(&pat_type.pat),
        syn::Pat::Wild(_) => "_".to_string(),
        syn::Pat::Tuple(pat_tuple) => {
            let names: Vec<String> = pat_tuple.elems.iter()
                .map(extract_pattern_name)
                .collect();
            format!("({})", names.join(", "))
        }
        syn::Pat::Struct(pat_struct) => {
            let struct_name = pat_struct.path.segments.iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            format!("{} {{ .. }}", struct_name)
        }
        _ => "param".to_string(),
    }
}

fn extract_where_clause(where_clause: &syn::WhereClause) -> String {
    let predicates: Vec<String> = where_clause.predicates.iter()
        .map(|predicate| {
            match predicate {
                syn::WherePredicate::Type(type_pred) => {
                    let bounded_ty = extract_type_name(&type_pred.bounded_ty);
                    let bounds: Vec<String> = type_pred.bounds.iter()
                        .map(|bound| {
                            match bound {
                                syn::TypeParamBound::Trait(trait_bound) => {
                                    trait_bound.path.segments.iter()
                                        .map(|s| s.ident.to_string())
                                        .collect::<Vec<_>>()
                                        .join("::")
                                }
                                syn::TypeParamBound::Lifetime(lifetime) => {
                                    lifetime.ident.to_string()
                                }
                                _ => "Bound".to_string(),
                            }
                        })
                        .collect();
                    format!("{}: {}", bounded_ty, bounds.join(" + "))
                }
                syn::WherePredicate::Lifetime(lifetime_pred) => {
                    let lifetime = &lifetime_pred.lifetime;
                    let bounds: Vec<String> = lifetime_pred.bounds.iter()
                        .map(|bound| bound.ident.to_string())
                        .collect();
                    if bounds.is_empty() {
                        lifetime.ident.to_string()
                    } else {
                        format!("{}: {}", lifetime.ident, bounds.join(" + "))
                    }
                }
                _ => "Where".to_string(),
            }
        })
        .collect();
    
    predicates.join(",\n    ")
}

fn extract_cfg_attributes(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut cfg_attrs = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("cfg") {
            // syn v2のparse_nested_metaを使って正確に解析
            let mut cfg_parts = Vec::new();
            
            let parse_result = attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    let ident_str = ident.to_string();
                    
                    // 値がある場合（target_pointer_width = "64" など）
                    if meta.input.peek(syn::Token![=]) {
                        let _eq = meta.input.parse::<syn::Token![=]>()?;
                        let value: syn::LitStr = meta.input.parse()?;
                        cfg_parts.push(format!("{} = \"{}\"", ident_str, value.value()));
                    } else {
                        // 単純なフラグ（unix, windows など）
                        cfg_parts.push(ident_str);
                    }
                } else {
                    // 複雑なパス（all, any など）
                    let path_str = meta.path.segments.iter()
                        .map(|s| s.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::");
                    cfg_parts.push(path_str);
                }
                Ok(())
            });
            
            // 解析に失敗した場合はフォールバック
            if parse_result.is_err() || cfg_parts.is_empty() {
                if let syn::Meta::List(meta_list) = &attr.meta {
                    cfg_parts.push(meta_list.tokens.to_string());
                }
            }
            
            if !cfg_parts.is_empty() {
                cfg_attrs.push(cfg_parts.join(", "));
            }
        }
    }
    
    cfg_attrs
}

fn extract_derives(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut derives = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("derive") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                // derive(Clone, Debug) のような形式をパース
                let derive_string = meta_list.tokens.to_string();
                // 簡単なパースで derive 名を抽出
                for part in derive_string.split(',') {
                    let derive_name = part.trim().to_string();
                    if !derive_name.is_empty() {
                        derives.push(derive_name);
                    }
                }
            }
        }
    }
    
    derives
}

impl<'a> CompleteDocsVisitor<'a> {
    pub fn extract_docs_for_item(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            if let Ok(meta) = attr.meta.require_name_value() {
                if meta.path.is_ident("doc") {
                    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) = &meta.value {
                        let doc_content = lit_str.value();
                        let doc_content = doc_content.trim();
                        
                        // docsコメント内の見出しレベルを調整
                        let adjusted_content = if doc_content.starts_with("# ") {
                            format!("#### {}", &doc_content[2..])
                        } else if doc_content.starts_with("## ") {
                            format!("##### {}", &doc_content[3..])
                        } else if doc_content.starts_with("### ") {
                            format!("###### {}", &doc_content[4..])
                        } else {
                            doc_content.to_string()
                        };
                        
                        self.content.push_str(&adjusted_content);
                        self.content.push('\n');
                    }
                }
            }
        }
        self.content.push('\n');
    }
}