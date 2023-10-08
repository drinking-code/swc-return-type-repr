use swc_core::ecma::{
    ast::*,
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        n.visit_mut_children_with(self)
    }

    fn visit_mut_class_method(&mut self, n: &mut ClassMethod) {
        println!("{:#?}", n.function.return_type);
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}