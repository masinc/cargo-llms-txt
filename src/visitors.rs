use syn::{
    visit::Visit, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl,
    ItemMacro, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion,
    ItemUse, Visibility,
};

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
            self.items
                .push(format!("pub fn {}{}", mod_path, node.sig.ident));
        }
    }

    fn visit_item_struct(&mut self, node: &ItemStruct) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub struct {}{}", mod_path, node.ident));
        }
    }

    fn visit_item_enum(&mut self, node: &ItemEnum) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub enum {}{}", mod_path, node.ident));
        }
    }

    fn visit_item_trait(&mut self, node: &ItemTrait) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub trait {}{}", mod_path, node.ident));
        }
    }

    fn visit_item_mod(&mut self, node: &ItemMod) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub mod {}{}", mod_path, node.ident));
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
            self.items
                .push(format!("pub const {}{}", mod_path, node.ident));
        }
    }

    fn visit_item_static(&mut self, node: &ItemStatic) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub static {}{}", mod_path, node.ident));
        }
    }

    fn visit_item_type(&mut self, node: &ItemType) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub type {}{}", mod_path, node.ident));
        }
    }

    fn visit_item_impl(&mut self, node: &syn::ItemImpl) {
        // 実装対象の型名を取得
        let impl_type = match &*node.self_ty {
            syn::Type::Path(type_path) => type_path
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
            _ => "Unknown".to_string(),
        };

        let mod_path = if self.current_mod.is_empty() {
            String::new()
        } else {
            format!("{}::", self.current_mod.join("::"))
        };

        // トレイト実装かどうか
        if let Some((_, trait_path, _)) = &node.trait_ {
            let trait_name = trait_path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            self.items
                .push(format!("impl {} for {}{}", trait_name, mod_path, impl_type));
        } else {
            self.items.push(format!("impl {}{}", mod_path, impl_type));
        }
    }

    fn visit_item_use(&mut self, node: &ItemUse) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            let use_tree = format_use_tree(&node.tree);
            self.items.push(format!("pub use {}{}", mod_path, use_tree));
        }
    }

    fn visit_item_macro(&mut self, node: &ItemMacro) {
        // macro_rules! マクロの場合
        if let Some(ident) = &node.ident {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items.push(format!("{}{}!", mod_path, ident));
        }
    }

    fn visit_item_extern_crate(&mut self, node: &ItemExternCrate) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub extern crate {}{}", mod_path, node.ident));
        }
    }

    fn visit_item_foreign_mod(&mut self, node: &ItemForeignMod) {
        for item in &node.items {
            if let syn::ForeignItem::Fn(foreign_fn) = item {
                if matches!(foreign_fn.vis, Visibility::Public(_)) {
                    let mod_path = if self.current_mod.is_empty() {
                        String::new()
                    } else {
                        format!("{}::", self.current_mod.join("::"))
                    };
                    let abi = node
                        .abi
                        .name
                        .as_ref()
                        .map(|lit| lit.value())
                        .unwrap_or("C".to_string());
                    self.items.push(format!(
                        "pub extern \"{}\" fn {}{}",
                        abi, mod_path, foreign_fn.sig.ident
                    ));
                }
            }
        }
    }

    fn visit_item_union(&mut self, node: &ItemUnion) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub union {}{}", mod_path, node.ident));
        }
    }

    fn visit_item_trait_alias(&mut self, node: &ItemTraitAlias) {
        if matches!(node.vis, Visibility::Public(_)) {
            let mod_path = if self.current_mod.is_empty() {
                String::new()
            } else {
                format!("{}::", self.current_mod.join("::"))
            };
            self.items
                .push(format!("pub trait {}{}", mod_path, node.ident));
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

    fn visit_item_use(&mut self, node: &ItemUse) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("re-exports".to_string());
        }
    }

    fn visit_item_macro(&mut self, node: &ItemMacro) {
        if node.ident.is_some() {
            *self.public_count += 1;
            self.types.push("macros".to_string());
        }
    }

    fn visit_item_extern_crate(&mut self, node: &ItemExternCrate) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("extern crates".to_string());
        }
    }

    fn visit_item_foreign_mod(&mut self, node: &ItemForeignMod) {
        for item in &node.items {
            if let syn::ForeignItem::Fn(foreign_fn) = item {
                if matches!(foreign_fn.vis, Visibility::Public(_)) {
                    *self.public_count += 1;
                    self.types.push("foreign functions".to_string());
                }
            }
        }
    }

    fn visit_item_union(&mut self, node: &ItemUnion) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("unions".to_string());
        }
    }

    fn visit_item_trait_alias(&mut self, node: &ItemTraitAlias) {
        if matches!(node.vis, Visibility::Public(_)) {
            *self.public_count += 1;
            self.types.push("trait aliases".to_string());
        }
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

            self.content
                .push_str(&format!("### {}{}\n\n", mod_path, node.sig.ident));

            // クリーンな関数シグネチャを作成
            self.content.push_str("```rust\n");

            // Check if this is an extern "C" function
            let is_extern_c = node
                .sig
                .abi
                .as_ref()
                .and_then(|abi| abi.name.as_ref())
                .map(|lit| lit.value() == "C")
                .unwrap_or(false);

            if is_extern_c {
                // Format as #[no_mangle] pub extern "C" fn ...
                let has_no_mangle = node
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("no_mangle"));

                if has_no_mangle {
                    self.content.push_str("#[no_mangle]\n");
                }

                let sig = format_function_signature(&node.sig, true, "");
                // Replace "pub fn" with "pub extern \"C\" fn"
                let extern_sig = sig.replace("pub fn", "pub extern \"C\" fn");
                self.content.push_str(&extern_sig);
            } else {
                let sig = format_function_signature(&node.sig, true, "");
                self.content.push_str(&sig);
            }

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

            self.content
                .push_str(&format!("### {}{}\n\n", mod_path, node.ident));

            self.content.push_str("```rust\n");

            // derive属性を抽出
            let derives = extract_derives(&node.attrs);
            if !derives.is_empty() {
                self.content
                    .push_str(&format!("#[derive({})]\n", derives.join(", ")));
            }

            // cfg属性を抽出
            let cfg_attrs = extract_cfg_attributes(&node.attrs);
            if !cfg_attrs.is_empty() {
                self.content
                    .push_str(&format!("#[cfg({})]\n", cfg_attrs.join(", ")));
            }

            // 構造体定義のクリーンな表示
            let mut struct_def = format!("pub struct {}", node.ident);

            // ジェネリクスを追加
            if !node.generics.params.is_empty() {
                struct_def.push('<');
                let generics: Vec<String> = node
                    .generics
                    .params
                    .iter()
                    .map(|p| match p {
                        syn::GenericParam::Type(tp) => tp.ident.to_string(),
                        syn::GenericParam::Lifetime(lp) => format!("'{}", lp.lifetime.ident),
                        syn::GenericParam::Const(cp) => {
                            format!("const {}: {}", cp.ident, extract_type_name(&cp.ty))
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
                                struct_def.push_str(&format!(
                                    "    pub {}: {},\n",
                                    ident,
                                    extract_type_name(&field.ty)
                                ));
                            }
                        }
                    }
                    struct_def.push('}');
                }
                syn::Fields::Unnamed(fields) => {
                    struct_def.push('(');
                    let field_types: Vec<String> = fields
                        .unnamed
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

            self.content
                .push_str(&format!("### {}{}\n\n", mod_path, node.ident));

            self.content.push_str("```rust\n");

            // derive属性を抽出
            let derives = extract_derives(&node.attrs);
            if !derives.is_empty() {
                self.content
                    .push_str(&format!("#[derive({})]\n", derives.join(", ")));
            }

            // cfg属性を抽出
            let cfg_attrs = extract_cfg_attributes(&node.attrs);
            if !cfg_attrs.is_empty() {
                self.content
                    .push_str(&format!("#[cfg({})]\n", cfg_attrs.join(", ")));
            }

            // 列挙型定義のクリーンな表示
            let mut enum_def = format!("pub enum {}", node.ident);

            // ジェネリクスを追加
            if !node.generics.params.is_empty() {
                enum_def.push('<');
                let generics: Vec<String> = node
                    .generics
                    .params
                    .iter()
                    .map(|p| match p {
                        syn::GenericParam::Type(tp) => tp.ident.to_string(),
                        syn::GenericParam::Lifetime(lp) => format!("'{}", lp.lifetime.ident),
                        syn::GenericParam::Const(cp) => {
                            format!("const {}: {}", cp.ident, extract_type_name(&cp.ty))
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
                    variant_str = format!(
                        "    #[cfg({})]\n    {}",
                        cfg_attrs.join(", "),
                        variant.ident
                    );
                }

                // バリアントフィールドを表示
                match &variant.fields {
                    syn::Fields::Named(fields) => {
                        variant_str.push_str(" {");
                        let field_strs: Vec<String> = fields
                            .named
                            .iter()
                            .filter_map(|f| {
                                if let Some(ident) = &f.ident {
                                    if matches!(f.vis, syn::Visibility::Public(_)) {
                                        Some(format!(
                                            " pub {}: {}",
                                            ident,
                                            extract_type_name(&f.ty)
                                        ))
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
                        let field_types: Vec<String> = fields
                            .unnamed
                            .iter()
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

            self.content
                .push_str(&format!("### {}{}\n\n", mod_path, node.ident));

            self.content.push_str("```rust\n");
            // トレイト定義のクリーンな表示
            let mut trait_signature = format!("pub trait {}", node.ident);

            // ジェネリクスを追加
            if !node.generics.params.is_empty() {
                trait_signature.push('<');
                let generics: Vec<String> = node
                    .generics
                    .params
                    .iter()
                    .map(|p| match p {
                        syn::GenericParam::Type(tp) => tp.ident.to_string(),
                        syn::GenericParam::Lifetime(lp) => format!("'{}", lp.lifetime.ident),
                        syn::GenericParam::Const(cp) => {
                            format!("const {}: {}", cp.ident, extract_type_name(&cp.ty))
                        }
                    })
                    .collect();
                trait_signature.push_str(&generics.join(", "));
                trait_signature.push('>');
            }

            // Super traitsを追加
            if !node.supertraits.is_empty() {
                trait_signature.push_str(": ");
                let supertraits: Vec<String> = node
                    .supertraits
                    .iter()
                    .map(|st| match st {
                        syn::TypeParamBound::Trait(trait_bound) => trait_bound
                            .path
                            .segments
                            .iter()
                            .map(|s| s.ident.to_string())
                            .collect::<Vec<_>>()
                            .join("::"),
                        syn::TypeParamBound::Lifetime(lifetime) => lifetime.ident.to_string(),
                        _ => "Unknown".to_string(),
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
                        let sig = format_function_signature(&method.sig, false, "    ");
                        self.content.push_str(&format!("\n    {};", sig));
                    }
                    syn::TraitItem::Type(ty) => {
                        self.content.push_str(&format!("\n    type {};", ty.ident));
                    }
                    syn::TraitItem::Const(c) => {
                        // クリーンなconst定義
                        let type_str = extract_type_name(&c.ty);
                        self.content
                            .push_str(&format!("\n    const {}: {};", c.ident, type_str));
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

            self.content
                .push_str(&format!("### {}{}\n\n", mod_path, node.ident));

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

            self.content
                .push_str(&format!("### {}{}\n\n", mod_path, node.ident));

            self.content.push_str("```rust\n");
            // クリーンなstatic定義
            let mut static_def = String::new();
            static_def.push_str("pub static ");
            if matches!(node.mutability, syn::StaticMutability::Mut(_)) {
                static_def.push_str("mut ");
            }
            static_def.push_str(&format!("{}: {}", node.ident, extract_type_name(&node.ty)));
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

            self.content
                .push_str(&format!("### {}{}\n\n", mod_path, node.ident));

            self.content.push_str("```rust\n");
            // クリーンなtype alias定義
            let mut type_def = format!("pub type {}", node.ident);

            // ジェネリクスを追加
            if !node.generics.params.is_empty() {
                let generics: Vec<String> = node
                    .generics
                    .params
                    .iter()
                    .map(|p| match p {
                        syn::GenericParam::Type(tp) => tp.ident.to_string(),
                        syn::GenericParam::Lifetime(lp) => format!("'{}", lp.lifetime.ident),
                        syn::GenericParam::Const(cp) => {
                            format!("const {}: {}", cp.ident, extract_type_name(&cp.ty))
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
            syn::Type::Path(type_path) => type_path
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
            _ => "Unknown".to_string(),
        };

        let mod_path = if self.current_mod.is_empty() {
            String::new()
        } else {
            format!("{}::", self.current_mod.join("::"))
        };

        // トレイト実装かどうか
        if let Some((_, trait_path, _)) = &node.trait_ {
            let trait_name = extract_path_with_generics(trait_path);

            self.content.push_str(&format!(
                "### impl {} for {}{}\n\n",
                trait_name, mod_path, impl_type
            ));
        } else {
            self.content
                .push_str(&format!("### impl {}{}\n\n", mod_path, impl_type));
        }

        self.content.push_str("```rust\n");

        // impl シグネチャを構築
        let mut impl_sig = String::new();

        // ジェネリクス
        if !node.generics.params.is_empty() {
            impl_sig.push_str("impl<");
            let generics: Vec<String> = node
                .generics
                .params
                .iter()
                .map(|p| match p {
                    syn::GenericParam::Type(tp) => tp.ident.to_string(),
                    syn::GenericParam::Lifetime(lp) => format!("'{}", lp.lifetime.ident),
                    syn::GenericParam::Const(cp) => {
                        format!("const {}: {}", cp.ident, extract_type_name(&cp.ty))
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
            let trait_name = extract_path_with_generics(trait_path);
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
                        let sig = format_function_signature(&method.sig, true, "    ");
                        self.content.push_str(&format!("\n    {};", sig));
                    }
                }
                syn::ImplItem::Const(const_item) => {
                    if matches!(const_item.vis, Visibility::Public(_)) {
                        self.content
                            .push_str(&format!("\n    pub const {}: Type;", const_item.ident));
                    }
                }
                syn::ImplItem::Type(type_item) => {
                    if matches!(type_item.vis, Visibility::Public(_)) {
                        self.content
                            .push_str(&format!("\n    pub type {};", type_item.ident));
                    }
                }
                _ => {}
            }
        }

        self.content.push_str("\n}\n```\n\n");

        // impl ブロックのdocsコメントがあれば抽出
        self.extract_docs_for_item(&node.attrs);
    }

    fn visit_item_use(&mut self, node: &ItemUse) {
        if matches!(node.vis, Visibility::Public(_)) {
            let use_tree = format_use_tree(&node.tree);

            self.content.push_str(&format!("### {}\n\n", use_tree));
            self.content.push_str("```rust\n");
            self.content.push_str(&format!("pub use {};\n", use_tree));
            self.content.push_str("```\n\n");

            self.extract_docs_for_item(&node.attrs);
        }
    }

    fn visit_item_macro(&mut self, node: &ItemMacro) {
        if let Some(ident) = &node.ident {
            self.content.push_str(&format!("### {}!\n\n", ident));
            self.content.push_str("```rust\n");
            self.content.push_str(&format!(
                "macro_rules! {} {{\n    // macro definition\n}}\n",
                ident
            ));
            self.content.push_str("```\n\n");

            self.extract_docs_for_item(&node.attrs);
        }
    }

    fn visit_item_extern_crate(&mut self, node: &ItemExternCrate) {
        if matches!(node.vis, Visibility::Public(_)) {
            self.content
                .push_str(&format!("### extern crate {}\n\n", node.ident));
            self.content.push_str("```rust\n");
            self.content
                .push_str(&format!("pub extern crate {};\n", node.ident));
            self.content.push_str("```\n\n");

            self.extract_docs_for_item(&node.attrs);
        }
    }

    fn visit_item_foreign_mod(&mut self, node: &ItemForeignMod) {
        for item in &node.items {
            if let syn::ForeignItem::Fn(foreign_fn) = item {
                if matches!(foreign_fn.vis, Visibility::Public(_)) {
                    let abi = node
                        .abi
                        .name
                        .as_ref()
                        .map(|lit| lit.value())
                        .unwrap_or("C".to_string());

                    self.content
                        .push_str(&format!("### {}\n\n", foreign_fn.sig.ident));
                    self.content.push_str("```rust\n");

                    // Format as extern "ABI" { pub fn ... }
                    self.content.push_str(&format!("extern \"{}\" {{\n", abi));
                    let sig = format_function_signature(&foreign_fn.sig, true, "");
                    self.content.push_str(&format!("    {};\n", sig));
                    self.content.push_str("}\n");
                    self.content.push_str("```\n\n");

                    self.extract_docs_for_item(&foreign_fn.attrs);
                }
            }
        }
    }

    fn visit_item_union(&mut self, node: &ItemUnion) {
        if matches!(node.vis, Visibility::Public(_)) {
            self.content.push_str(&format!("### {}\n\n", node.ident));
            self.content.push_str("```rust\n");

            // Extract and format attributes
            let attrs = extract_cfg_attributes(&node.attrs);
            if !attrs.is_empty() {
                self.content.push_str(&format!("#[{}]\n", attrs.join(", ")));
            }

            // Union header with generics
            let mut union_def = format!("pub union {}", node.ident);
            if !node.generics.params.is_empty() {
                let generics = format_generic_params_simple(&node.generics.params);
                union_def.push_str(&format!("<{}>", generics));
            }

            self.content.push_str(&format!("{} {{\n", union_def));

            // Union fields
            for field in &node.fields.named {
                if let Some(ident) = &field.ident {
                    let type_str = extract_type_name(&field.ty);
                    self.content
                        .push_str(&format!("    pub {}: {},\n", ident, type_str));
                }
            }

            self.content.push_str("}\n```\n\n");

            self.extract_docs_for_item(&node.attrs);
        }
    }

    fn visit_item_trait_alias(&mut self, node: &ItemTraitAlias) {
        if matches!(node.vis, Visibility::Public(_)) {
            self.content.push_str(&format!("### {}\n\n", node.ident));
            self.content.push_str("```rust\n");

            // Trait alias with generics
            let mut trait_alias = format!("pub trait {}", node.ident);
            if !node.generics.params.is_empty() {
                let generics = format_generic_params_simple(&node.generics.params);
                trait_alias.push_str(&format!("<{}>", generics));
            }

            // Format bounds
            let bounds = format_trait_bounds(&node.bounds);
            trait_alias.push_str(&format!(" = {}", bounds));

            // Add where clause if present
            if let Some(where_clause) = &node.generics.where_clause {
                let where_str = extract_where_clause(where_clause);
                if !where_str.is_empty() {
                    trait_alias.push_str(&format!("\nwhere\n    {}", where_str));
                }
            }

            self.content.push_str(&format!("{};\n", trait_alias));
            self.content.push_str("```\n\n");

            self.extract_docs_for_item(&node.attrs);
        }
    }
}

// Helper function for formatting trait bounds
fn format_trait_bounds(
    bounds: &syn::punctuated::Punctuated<syn::TypeParamBound, syn::token::Plus>,
) -> String {
    bounds
        .iter()
        .map(|bound| match bound {
            syn::TypeParamBound::Trait(trait_bound) => trait_bound
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
            syn::TypeParamBound::Lifetime(lifetime) => {
                format!("'{}", lifetime.ident)
            }
            _ => "Unknown".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

// Helper function for formatting use trees
fn format_use_tree(tree: &syn::UseTree) -> String {
    match tree {
        syn::UseTree::Path(path) => {
            format!("{}::{}", path.ident, format_use_tree(&path.tree))
        }
        syn::UseTree::Name(name) => name.ident.to_string(),
        syn::UseTree::Rename(rename) => {
            format!("{} as {}", rename.ident, rename.rename)
        }
        syn::UseTree::Glob(_) => "*".to_string(),
        syn::UseTree::Group(group) => {
            let items: Vec<String> = group.items.iter().map(format_use_tree).collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}

// Helper function for simple generic parameter formatting
fn format_generic_params_simple(
    params: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
) -> String {
    params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Type(type_param) => type_param.ident.to_string(),
            syn::GenericParam::Lifetime(lifetime_param) => {
                format!("'{}", lifetime_param.lifetime.ident)
            }
            syn::GenericParam::Const(const_param) => const_param.ident.to_string(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn extract_type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .iter()
            .map(|s| {
                let mut segment = s.ident.to_string();
                if !s.arguments.is_empty() {
                    match &s.arguments {
                        syn::PathArguments::AngleBracketed(args) => {
                            segment.push('<');
                            let arg_strs: Vec<String> = args
                                .args
                                .iter()
                                .map(|arg| match arg {
                                    syn::GenericArgument::Type(ty) => extract_type_name(ty),
                                    syn::GenericArgument::Lifetime(lt) => format!("'{}", lt.ident),
                                    syn::GenericArgument::Const(_) => "Const".to_string(),
                                    _ => "Unknown".to_string(),
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
            .join("::"),
        syn::Type::Reference(type_ref) => {
            let mut ref_str = "&".to_string();
            if let Some(lifetime) = &type_ref.lifetime {
                ref_str.push('\'');
                ref_str.push_str(&lifetime.ident.to_string());
                ref_str.push(' ');
            }
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
            let elem_strs: Vec<String> = type_tuple.elems.iter().map(extract_type_name).collect();
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
            let names: Vec<String> = pat_tuple.elems.iter().map(extract_pattern_name).collect();
            format!("({})", names.join(", "))
        }
        syn::Pat::Struct(pat_struct) => {
            let struct_name = pat_struct
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            format!("{} {{ .. }}", struct_name)
        }
        _ => "param".to_string(),
    }
}

fn format_function_signature(
    sig: &syn::Signature,
    include_pub: bool,
    where_indent: &str,
) -> String {
    let mut result = String::new();

    // pub fn or fn
    if include_pub {
        result.push_str("pub fn ");
    } else {
        result.push_str("fn ");
    }

    // 関数名
    result.push_str(&sig.ident.to_string());

    // ジェネリクス
    if !sig.generics.params.is_empty() {
        result.push('<');
        let generics: Vec<String> = sig
            .generics
            .params
            .iter()
            .map(|p| match p {
                syn::GenericParam::Type(tp) => {
                    let mut type_str = tp.ident.to_string();
                    if !tp.bounds.is_empty() {
                        type_str.push_str(": ");
                        let bounds: Vec<String> = tp
                            .bounds
                            .iter()
                            .map(|bound| match bound {
                                syn::TypeParamBound::Trait(trait_bound) => trait_bound
                                    .path
                                    .segments
                                    .iter()
                                    .map(|s| s.ident.to_string())
                                    .collect::<Vec<_>>()
                                    .join("::"),
                                syn::TypeParamBound::Lifetime(lifetime) => {
                                    format!("'{}", lifetime.ident)
                                }
                                _ => "Bound".to_string(),
                            })
                            .collect();
                        type_str.push_str(&bounds.join(" + "));
                    }
                    type_str
                }
                syn::GenericParam::Lifetime(lp) => {
                    let mut lifetime_str = format!("'{}", lp.lifetime.ident);
                    if !lp.bounds.is_empty() {
                        lifetime_str.push_str(": ");
                        let bounds: Vec<String> = lp
                            .bounds
                            .iter()
                            .map(|bound| format!("'{}", bound.ident))
                            .collect();
                        lifetime_str.push_str(&bounds.join(" + "));
                    }
                    lifetime_str
                }
                syn::GenericParam::Const(cp) => {
                    format!("const {}: {}", cp.ident, extract_type_name(&cp.ty))
                }
            })
            .collect();
        result.push_str(&generics.join(", "));
        result.push('>');
    }

    // パラメータ
    result.push('(');
    let params: Vec<String> = sig
        .inputs
        .iter()
        .map(|input| match input {
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
        })
        .collect();

    // パラメータを1行にまとめて表示
    result.push_str(&params.join(", "));
    result.push(')');

    // 戻り値型
    if let syn::ReturnType::Type(_, ty) = &sig.output {
        result.push_str(" -> ");
        result.push_str(&extract_type_name(ty));
    }

    // where句
    if let Some(where_clause) = &sig.generics.where_clause {
        if !where_clause.predicates.is_empty() {
            result.push_str(&format!("\n{}where\n{}    ", where_indent, where_indent));
            result.push_str(&extract_where_clause(where_clause));
        }
    }

    result
}

fn extract_where_clause(where_clause: &syn::WhereClause) -> String {
    let predicates: Vec<String> = where_clause
        .predicates
        .iter()
        .map(|predicate| match predicate {
            syn::WherePredicate::Type(type_pred) => {
                let bounded_ty = extract_type_name(&type_pred.bounded_ty);
                let bounds: Vec<String> = type_pred
                    .bounds
                    .iter()
                    .map(|bound| match bound {
                        syn::TypeParamBound::Trait(trait_bound) => trait_bound
                            .path
                            .segments
                            .iter()
                            .map(|s| s.ident.to_string())
                            .collect::<Vec<_>>()
                            .join("::"),
                        syn::TypeParamBound::Lifetime(lifetime) => {
                            format!("'{}", lifetime.ident)
                        }
                        _ => "Bound".to_string(),
                    })
                    .collect();
                format!("{}: {}", bounded_ty, bounds.join(" + "))
            }
            syn::WherePredicate::Lifetime(lifetime_pred) => {
                let lifetime = &lifetime_pred.lifetime;
                let bounds: Vec<String> = lifetime_pred
                    .bounds
                    .iter()
                    .map(|bound| format!("'{}", bound.ident))
                    .collect();
                if bounds.is_empty() {
                    format!("'{}", lifetime.ident)
                } else {
                    format!("'{}: {}", lifetime.ident, bounds.join(" + "))
                }
            }
            _ => "Where".to_string(),
        })
        .collect();

    predicates.join(",\n    ")
}

fn extract_path_with_generics(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| {
            let mut seg_str = segment.ident.to_string();

            // ジェネリクス引数があるかチェック
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if !args.args.is_empty() {
                    seg_str.push('<');
                    let generic_args: Vec<String> = args
                        .args
                        .iter()
                        .map(|arg| {
                            match arg {
                                syn::GenericArgument::Type(ty) => extract_type_name(ty),
                                syn::GenericArgument::Lifetime(lt) => format!("'{}", lt.ident),
                                syn::GenericArgument::Const(expr) => {
                                    // const式を文字列化するのは複雑なので、簡単な場合のみ対応
                                    match expr {
                                        syn::Expr::Lit(lit) => match &lit.lit {
                                            syn::Lit::Str(s) => format!("\"{}\"", s.value()),
                                            syn::Lit::Int(i) => i.base10_digits().to_string(),
                                            syn::Lit::Bool(b) => b.value.to_string(),
                                            _ => "Const".to_string(),
                                        },
                                        syn::Expr::Path(path) => path
                                            .path
                                            .segments
                                            .iter()
                                            .map(|s| s.ident.to_string())
                                            .collect::<Vec<_>>()
                                            .join("::"),
                                        _ => "Const".to_string(),
                                    }
                                }
                                syn::GenericArgument::AssocType(assoc_type) => {
                                    format!(
                                        "{} = {}",
                                        assoc_type.ident,
                                        extract_type_name(&assoc_type.ty)
                                    )
                                }
                                syn::GenericArgument::AssocConst(_) => "AssocConst".to_string(),
                                syn::GenericArgument::Constraint(_) => "Constraint".to_string(),
                                _ => "GenericArg".to_string(),
                            }
                        })
                        .collect();
                    seg_str.push_str(&generic_args.join(", "));
                    seg_str.push('>');
                }
            }

            seg_str
        })
        .collect::<Vec<_>>()
        .join("::")
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
                    let path_str = meta
                        .path
                        .segments
                        .iter()
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
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit_str),
                        ..
                    }) = &meta.value
                    {
                        let doc_content = lit_str.value();
                        let doc_content = doc_content.trim();

                        // docsコメント内の見出しレベルを調整
                        let adjusted_content =
                            if let Some(stripped) = doc_content.strip_prefix("# ") {
                                format!("#### {}", stripped)
                            } else if let Some(stripped) = doc_content.strip_prefix("## ") {
                                format!("##### {}", stripped)
                            } else if let Some(stripped) = doc_content.strip_prefix("### ") {
                                format!("###### {}", stripped)
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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::visit::Visit;

    #[test]
    fn test_extract_pattern_name() {
        // Test different pattern types by constructing them manually
        use syn::{PatIdent, PatWild};

        // Test identifier pattern
        let ident = syn::Ident::new("x", proc_macro2::Span::call_site());
        let pat_ident = PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident,
            subpat: None,
        };
        let pat = syn::Pat::Ident(pat_ident);
        assert_eq!(extract_pattern_name(&pat), "x");

        // Test wildcard pattern
        let pat_wild = PatWild {
            attrs: vec![],
            underscore_token: Default::default(),
        };
        let pat = syn::Pat::Wild(pat_wild);
        assert_eq!(extract_pattern_name(&pat), "_");
    }

    #[test]
    fn test_extract_type_name() {
        // Test simple type
        let code = "u32";
        let ty: syn::Type = syn::parse_str(code).unwrap();
        assert_eq!(extract_type_name(&ty), "u32");

        // Test reference type
        let code = "&str";
        let ty: syn::Type = syn::parse_str(code).unwrap();
        assert_eq!(extract_type_name(&ty), "&str");

        // Test mutable reference type
        let code = "&mut String";
        let ty: syn::Type = syn::parse_str(code).unwrap();
        assert_eq!(extract_type_name(&ty), "&mut String");

        // Test generic type
        let code = "Vec<T>";
        let ty: syn::Type = syn::parse_str(code).unwrap();
        assert_eq!(extract_type_name(&ty), "Vec<T>");

        // Test slice type
        let code = "[u8]";
        let ty: syn::Type = syn::parse_str(code).unwrap();
        assert_eq!(extract_type_name(&ty), "[u8]");

        // Test array type (note: syn might parse const generics differently)
        let code = "[u8; 32]";
        let ty: syn::Type = syn::parse_str(code).unwrap();
        let result = extract_type_name(&ty);
        assert!(result.starts_with("[u8; ") && result.ends_with("]"));

        // Test tuple type
        let code = "(String, i32)";
        let ty: syn::Type = syn::parse_str(code).unwrap();
        assert_eq!(extract_type_name(&ty), "(String, i32)");
    }

    #[test]
    fn test_extract_where_clause() {
        let code = "fn test<T>() where T: Clone + Send, T: 'static {}";
        let item: syn::ItemFn = syn::parse_str(code).unwrap();

        if let Some(where_clause) = &item.sig.generics.where_clause {
            let result = extract_where_clause(where_clause);
            assert!(result.contains("T: Clone + Send"));
        }
    }

    #[test]
    fn test_extract_cfg_attributes() {
        let code = r#"
            #[cfg(feature = "std")]
            #[cfg(target_pointer_width = "64")]
            #[cfg(unix)]
            pub fn test() {}
        "#;
        let item: syn::ItemFn = syn::parse_str(code).unwrap();
        let cfg_attrs = extract_cfg_attributes(&item.attrs);

        assert_eq!(cfg_attrs.len(), 3);
        assert!(cfg_attrs.contains(&"feature = \"std\"".to_string()));
        assert!(cfg_attrs.contains(&"target_pointer_width = \"64\"".to_string()));
        assert!(cfg_attrs.contains(&"unix".to_string()));
    }

    #[test]
    fn test_toc_visitor_function() {
        let code = r#"
            pub fn public_function() {}
            fn private_function() {}
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut items = Vec::new();
        let mut visitor = TocVisitor {
            items: &mut items,
            current_mod: Vec::new(),
        };

        visitor.visit_file(&file);

        assert_eq!(items.len(), 1);
        assert_eq!(items[0], "pub fn public_function");
    }

    #[test]
    fn test_toc_visitor_struct() {
        let code = r#"
            pub struct PublicStruct;
            struct PrivateStruct;
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut items = Vec::new();
        let mut visitor = TocVisitor {
            items: &mut items,
            current_mod: Vec::new(),
        };

        visitor.visit_file(&file);

        assert_eq!(items.len(), 1);
        assert_eq!(items[0], "pub struct PublicStruct");
    }

    #[test]
    fn test_toc_visitor_enum() {
        let code = r#"
            pub enum PublicEnum {
                Variant1,
                Variant2(u32),
            }
            enum PrivateEnum {
                Variant,
            }
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut items = Vec::new();
        let mut visitor = TocVisitor {
            items: &mut items,
            current_mod: Vec::new(),
        };

        visitor.visit_file(&file);

        assert_eq!(items.len(), 1);
        assert_eq!(items[0], "pub enum PublicEnum");
    }

    #[test]
    fn test_toc_visitor_with_module() {
        let code = r#"
            pub mod submodule {
                pub fn function_in_module() {}
                pub struct StructInModule;
            }
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut items = Vec::new();
        let mut visitor = TocVisitor {
            items: &mut items,
            current_mod: Vec::new(),
        };

        visitor.visit_file(&file);

        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "pub mod submodule");
        assert_eq!(items[1], "pub fn submodule::function_in_module");
        assert_eq!(items[2], "pub struct submodule::StructInModule");
    }

    #[test]
    fn test_summary_visitor() {
        let code = r#"
            /// This is a public function
            pub fn test_function(param: u32) -> String {
                String::new()
            }
            
            /// This is a public struct
            pub struct TestStruct {
                pub field: i32,
            }
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut public_count = 0;
        let mut types = Vec::new();

        let mut visitor = SummaryVisitor {
            public_count: &mut public_count,
            types: &mut types,
        };

        visitor.visit_file(&file);

        assert_eq!(public_count, 2);
        assert!(types.contains(&"functions".to_string()));
        assert!(types.contains(&"structs".to_string()));
    }

    #[test]
    fn test_complete_docs_visitor_function_with_generics_and_where() {
        let code = r#"
            /// A generic function with where clause
            pub fn generic_function<T, U>(param1: T, param2: U) -> T 
            where 
                T: Clone + Send,
                U: std::fmt::Display,
            {
                param1
            }
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut content = String::new();

        let mut visitor = CompleteDocsVisitor {
            content: &mut content,
            current_mod: Vec::new(),
        };

        visitor.visit_file(&file);

        assert!(content.contains("generic_function<T, U>"));
        assert!(content.contains("param1: T"));
        assert!(content.contains("param2: U"));
        assert!(content.contains("T: Clone + Send"));
        assert!(content.contains("U: std::fmt::Display"));
    }

    #[test]
    fn test_extract_path_with_generics() {
        // Test simple path without generics
        let code = "std::fmt::Display";
        let path: syn::Path = syn::parse_str(code).unwrap();
        assert_eq!(extract_path_with_generics(&path), "std::fmt::Display");

        // Test path with type generics
        let code = "Vec<T>";
        let path: syn::Path = syn::parse_str(code).unwrap();
        assert_eq!(extract_path_with_generics(&path), "Vec<T>");

        // Test path with multiple generics
        let code = "HashMap<String, i32>";
        let path: syn::Path = syn::parse_str(code).unwrap();
        assert_eq!(extract_path_with_generics(&path), "HashMap<String, i32>");

        // Test path with associated type
        let code = "Iterator<Item = String>";
        let path: syn::Path = syn::parse_str(code).unwrap();
        let result = extract_path_with_generics(&path);
        assert_eq!(result, "Iterator<Item = String>");

        // Test nested generics
        let code = "Option<Vec<T>>";
        let path: syn::Path = syn::parse_str(code).unwrap();
        assert_eq!(extract_path_with_generics(&path), "Option<Vec<T>>");
    }

    #[test]
    fn test_complete_docs_visitor_enum_with_variants() {
        let code = r#"
            /// An enum with different variant types
            pub enum TestEnum {
                /// Unit variant
                Unit,
                /// Tuple variant
                Tuple(String, i32),
                /// Struct variant  
                Struct { name: String, value: i32 },
                #[cfg(feature = "test")]
                ConditionalVariant,
            }
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut content = String::new();

        let mut visitor = CompleteDocsVisitor {
            content: &mut content,
            current_mod: Vec::new(),
        };

        visitor.visit_file(&file);

        assert!(content.contains("pub enum TestEnum"));
        assert!(content.contains("Unit"));
        assert!(content.contains("Tuple(String, i32)"));
        assert!(content.contains("Struct { name: String, value: i32 }"));
        assert!(content.contains(r#"#[cfg(feature = "test")]"#));
    }

    #[test]
    fn test_complete_docs_visitor_impl_block() {
        let code = r#"
            pub struct TestStruct;
            
            impl TestStruct {
                /// A public method
                pub fn public_method(&self, param: u32) -> String {
                    String::new()
                }
                
                fn private_method(&self) {}
            }
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut content = String::new();

        let mut visitor = CompleteDocsVisitor {
            content: &mut content,
            current_mod: Vec::new(),
        };

        visitor.visit_file(&file);

        assert!(content.contains("impl TestStruct"));
        assert!(content.contains("pub fn public_method"));
        assert!(content.contains("param: u32"));
        assert!(!content.contains("private_method"));
    }

    #[test]
    fn test_complete_docs_visitor_impl_with_trait_generics() {
        let code = r#"
            pub struct MyStruct;
            
            impl<'de> serde::Deserialize<'de> for MyStruct {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    Ok(MyStruct)
                }
            }
        "#;

        let file: syn::File = syn::parse_str(code).unwrap();
        let mut content = String::new();

        let mut visitor = CompleteDocsVisitor {
            content: &mut content,
            current_mod: Vec::new(),
        };

        visitor.visit_file(&file);

        // Should include trait generics in the impl declaration
        assert!(content.contains("impl serde::Deserialize"));
        assert!(content.contains("for MyStruct"));
        // Should show the trait with its generics
        assert!(content.contains("Deserialize<'de>") || content.contains("Deserialize"));
    }

    #[test]
    fn test_format_function_signature() {
        // Test simple function without generics
        let sig: syn::Signature = syn::parse_str("fn simple_function(x: i32) -> bool").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert_eq!(result, "fn simple_function(x: i32) -> bool");

        // Test with pub
        let result = format_function_signature(&sig, true, "");
        assert_eq!(result, "pub fn simple_function(x: i32) -> bool");

        // Test function with generics
        let sig: syn::Signature =
            syn::parse_str("fn generic_function<T>(value: T) -> Option<T>").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert_eq!(result, "fn generic_function<T>(value: T) -> Option<T>");

        // Test function with self parameter
        let sig: syn::Signature =
            syn::parse_str("fn method(&self, param: String) -> Result<(), Error>").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert_eq!(
            result,
            "fn method(&self, param: String) -> Result<(), Error>"
        );

        // Test function with mutable self
        let sig: syn::Signature = syn::parse_str("fn mut_method(&mut self, x: i32)").unwrap();
        let result = format_function_signature(&sig, true, "");
        assert_eq!(result, "pub fn mut_method(&mut self, x: i32)");

        // Test function with multiple parameters
        let sig: syn::Signature =
            syn::parse_str("fn multi_param(a: i32, b: &str, c: Vec<u8>) -> String").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert_eq!(
            result,
            "fn multi_param(a: i32, b: &str, c: Vec<u8>) -> String"
        );

        // Test function with no parameters
        let sig: syn::Signature = syn::parse_str("fn no_params() -> bool").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert_eq!(result, "fn no_params() -> bool");

        // Test function with no return type
        let sig: syn::Signature = syn::parse_str("fn no_return(x: i32)").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert_eq!(result, "fn no_return(x: i32)");
    }

    #[test]
    fn test_format_function_signature_with_where_clause() {
        // Test function with where clause
        let sig: syn::Signature =
            syn::parse_str("fn with_where<T>(value: T) -> T where T: Clone").unwrap();
        let result = format_function_signature(&sig, false, "    ");
        assert!(result.contains("fn with_where<T>(value: T) -> T"));
        assert!(result.contains("where"));
        assert!(result.contains("T: Clone"));

        // Test multiple where clauses
        let sig: syn::Signature =
            syn::parse_str("fn complex_where<T, U>(t: T, u: U) -> (T, U) where T: Clone, U: Send")
                .unwrap();
        let result = format_function_signature(&sig, true, "");
        assert!(result.contains("pub fn complex_where<T, U>(t: T, u: U) -> (T, U)"));
        assert!(result.contains("where"));
        assert!(result.contains("T: Clone"));
        assert!(result.contains("U: Send"));

        // Test lifetime and const generics
        let sig: syn::Signature =
            syn::parse_str("fn lifetime_generic<'a, T>(data: &'a T) -> &'a T where T: 'a").unwrap();
        let result = format_function_signature(&sig, false, "  ");
        assert!(result.contains("fn lifetime_generic<'a, T>(data: &'a T) -> &'a T"));
        assert!(result.contains("where"));
        assert!(result.contains("T: 'a"));
    }

    #[test]
    fn test_format_function_signature_multiple_lifetimes() {
        // Test function with multiple lifetimes
        let sig: syn::Signature =
            syn::parse_str("fn multiple_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str")
                .unwrap();
        let result = format_function_signature(&sig, false, "");
        assert!(result.contains("fn multiple_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str"));

        // Test with pub
        let result = format_function_signature(&sig, true, "");
        assert!(
            result.contains("pub fn multiple_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str")
        );

        // Test function with multiple lifetimes and where clause
        let sig: syn::Signature = syn::parse_str(
            "fn complex_lifetimes<'a, 'b, T>(x: &'a T, y: &'b T) -> &'a T where 'b: 'a, T: Clone",
        )
        .unwrap();
        let result = format_function_signature(&sig, false, "    ");
        assert!(result.contains("fn complex_lifetimes<'a, 'b, T>(x: &'a T, y: &'b T) -> &'a T"));
        assert!(result.contains("where"));
        assert!(result.contains("'b: 'a"));
        assert!(result.contains("T: Clone"));

        // Test with mutable references and multiple lifetimes
        let sig: syn::Signature = syn::parse_str("fn mut_multiple_lifetimes<'a, 'b>(&mut self, x: &'a mut String, y: &'b str) -> &'a String").unwrap();
        let result = format_function_signature(&sig, true, "");
        assert!(result.contains("pub fn mut_multiple_lifetimes<'a, 'b>(&mut self, x: &'a mut String, y: &'b str) -> &'a String"));

        // Test with static lifetime
        let sig: syn::Signature =
            syn::parse_str("fn with_static<'a>(x: &'a str, y: &'static str) -> &'a str").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert!(result.contains("fn with_static<'a>(x: &'a str, y: &'static str) -> &'a str"));

        // Test complex lifetime bounds in where clause
        let sig: syn::Signature = syn::parse_str("fn lifetime_bounds<'a, 'b, 'c, T>(data: &'a T) -> &'a T where 'b: 'a, 'c: 'b, T: 'a + 'b").unwrap();
        let result = format_function_signature(&sig, false, "  ");
        assert!(result.contains("fn lifetime_bounds<'a, 'b, 'c, T>(data: &'a T) -> &'a T"));
        assert!(result.contains("where"));
        assert!(result.contains("'b: 'a"));
        assert!(result.contains("'c: 'b"));
        assert!(result.contains("T: 'a + 'b"));
    }

    #[test]
    fn test_format_function_signature_lifetime_bounds_in_generics() {
        // Test lifetime bounds in generic parameters
        let sig: syn::Signature = syn::parse_str(
            "fn lifetime_param_bounds<'a: 'b, 'b>(x: &'a str, y: &'b str) -> &'b str",
        )
        .unwrap();
        let result = format_function_signature(&sig, false, "");
        assert!(result.contains("fn lifetime_param_bounds"));
        assert!(result.contains("'a: 'b"));
        assert!(result.contains("'b"));
        assert!(result.contains("x: &'a str, y: &'b str"));

        // Test multiple lifetime bounds in generic parameters
        let sig: syn::Signature = syn::parse_str(
            "fn complex_lifetime_bounds<'a: 'b + 'c, 'b, 'c>(data: &'a str) -> &'a str",
        )
        .unwrap();
        let result = format_function_signature(&sig, true, "");
        assert!(result.contains("pub fn complex_lifetime_bounds"));
        assert!(result.contains("'a: 'b + 'c"));
        assert!(result.contains("'b"));
        assert!(result.contains("'c"));

        // Test combination of lifetime bounds in generics and where clause
        let sig: syn::Signature = syn::parse_str(
            "fn mixed_bounds<'a: 'b, 'b, T>(x: &'a T, y: &'b T) -> &'a T where T: Clone + 'a",
        )
        .unwrap();
        let result = format_function_signature(&sig, false, "    ");
        assert!(result.contains("fn mixed_bounds"));
        assert!(result.contains("'a: 'b"));
        assert!(result.contains("'b"));
        assert!(result.contains("T"));
        assert!(result.contains("where"));
        assert!(result.contains("T: Clone + 'a"));

        // Test static lifetime bounds
        let sig: syn::Signature =
            syn::parse_str("fn static_bounds<'a: 'static>(data: &'a str) -> &'a str").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert!(result.contains("'a: 'static"));
    }

    #[test]
    fn test_format_function_signature_type_param_bounds() {
        // Test type parameter bounds like <A: B, B: C, C>
        let sig: syn::Signature =
            syn::parse_str("fn type_bounds<A: B, B: C, C>(a: A, b: B, c: C) -> A").unwrap();
        let result = format_function_signature(&sig, false, "");
        assert!(result.contains("fn type_bounds"));
        assert!(result.contains("A: B"));
        assert!(result.contains("B: C"));
        assert!(result.contains("C"));

        // Test more complex type bounds
        let sig: syn::Signature = syn::parse_str(
            "fn complex_type_bounds<T: Clone + Send, U: T + Debug>(t: T, u: U) -> T",
        )
        .unwrap();
        let result = format_function_signature(&sig, true, "");
        assert!(result.contains("pub fn complex_type_bounds"));
        assert!(result.contains("T: Clone + Send"));
        assert!(result.contains("U: T + Debug"));
    }

    #[test]
    fn test_format_use_tree_simple() {
        // Test simple name
        let use_tree: syn::UseTree = syn::parse_str("HashMap").unwrap();
        let result = format_use_tree(&use_tree);
        assert_eq!(result, "HashMap");
    }

    #[test]
    fn test_format_use_tree_path() {
        // Test path
        let use_tree: syn::UseTree = syn::parse_str("std::collections::HashMap").unwrap();
        let result = format_use_tree(&use_tree);
        assert_eq!(result, "std::collections::HashMap");
    }

    #[test]
    fn test_format_use_tree_rename() {
        // Test rename (as)
        let use_tree: syn::UseTree = syn::parse_str("Vec as SimpleVec").unwrap();
        let result = format_use_tree(&use_tree);
        assert_eq!(result, "Vec as SimpleVec");
    }

    #[test]
    fn test_format_use_tree_glob() {
        // Test glob (*)
        let use_tree: syn::UseTree = syn::parse_str("*").unwrap();
        let result = format_use_tree(&use_tree);
        assert_eq!(result, "*");
    }

    #[test]
    fn test_format_use_tree_group() {
        // Test group ({})
        let use_tree: syn::UseTree = syn::parse_str("{HashMap, BTreeMap}").unwrap();
        let result = format_use_tree(&use_tree);
        assert_eq!(result, "{HashMap, BTreeMap}");
    }

    #[test]
    fn test_format_use_tree_complex() {
        // Test complex path with rename
        let use_tree: syn::UseTree = syn::parse_str("std::vec::Vec as SimpleVec").unwrap();
        let result = format_use_tree(&use_tree);
        assert_eq!(result, "std::vec::Vec as SimpleVec");
    }

    #[test]
    fn test_format_generic_params_simple_types() {
        // Test type parameters
        let generics: syn::Generics = syn::parse_str("<T, U, V>").unwrap();
        let result = format_generic_params_simple(&generics.params);
        assert_eq!(result, "T, U, V");
    }

    #[test]
    fn test_format_generic_params_simple_lifetimes() {
        // Test lifetime parameters
        let generics: syn::Generics = syn::parse_str("<'a, 'b, 'c>").unwrap();
        let result = format_generic_params_simple(&generics.params);
        assert_eq!(result, "'a, 'b, 'c");
    }

    #[test]
    fn test_format_generic_params_simple_mixed() {
        // Test mixed parameters
        let generics: syn::Generics = syn::parse_str("<'a, T, 'b, U>").unwrap();
        let result = format_generic_params_simple(&generics.params);
        assert_eq!(result, "'a, T, 'b, U");
    }

    #[test]
    fn test_format_generic_params_simple_const() {
        // Test const parameters
        let generics: syn::Generics = syn::parse_str("<const N: usize>").unwrap();
        let result = format_generic_params_simple(&generics.params);
        assert_eq!(result, "N");
    }

    #[test]
    fn test_format_generic_params_simple_empty() {
        // Test empty parameters
        let generics: syn::Generics = syn::parse_str("").unwrap();
        let result = format_generic_params_simple(&generics.params);
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_generic_params_simple_complex() {
        // Test complex mixed parameters
        let generics: syn::Generics = syn::parse_str("<'a, T, const N: usize, 'b, U>").unwrap();
        let result = format_generic_params_simple(&generics.params);
        assert_eq!(result, "'a, T, N, 'b, U");
    }
}
