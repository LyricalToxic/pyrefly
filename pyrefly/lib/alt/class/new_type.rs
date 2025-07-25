/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use pyrefly_python::dunder;
use ruff_python_ast::name::Name;
use starlark_map::smallmap;

use crate::alt::answers::LookupAnswer;
use crate::alt::answers_solver::AnswersSolver;
use crate::alt::types::class_metadata::ClassSynthesizedField;
use crate::alt::types::class_metadata::ClassSynthesizedFields;
use crate::types::callable::Callable;
use crate::types::callable::FuncMetadata;
use crate::types::callable::Function;
use crate::types::callable::Param;
use crate::types::callable::ParamList;
use crate::types::callable::Required;
use crate::types::class::Class;
use crate::types::class::ClassType;
use crate::types::types::Type;

impl<'a, Ans: LookupAnswer> AnswersSolver<'a, Ans> {
    fn get_new_type_init(&self, cls: &Class, base: ClassType) -> ClassSynthesizedField {
        let params = vec![
            self.class_self_param(cls, false),
            Param::Pos(Name::new_static("_x"), base.to_type(), Required::Required),
        ];
        let ty = Type::Function(Box::new(Function {
            signature: Callable::list(ParamList::new(params), self.instantiate(cls)),
            metadata: FuncMetadata::def(self.module().name(), cls.name().clone(), dunder::INIT),
        }));
        ClassSynthesizedField::new(ty)
    }

    fn get_new_type_new(&self, cls: &Class, base: ClassType) -> ClassSynthesizedField {
        let params = vec![
            Param::Pos(
                Name::new_static("cls"),
                Type::type_form(self.instantiate(cls)),
                Required::Required,
            ),
            Param::Pos(Name::new_static("_x"), base.to_type(), Required::Required),
        ];
        let ty = Type::Function(Box::new(Function {
            signature: Callable::list(ParamList::new(params), self.instantiate(cls)),
            metadata: FuncMetadata::def(self.module().name(), cls.name().clone(), dunder::NEW),
        }));
        ClassSynthesizedField::new(ty)
    }

    pub fn get_new_type_synthesized_fields(&self, cls: &Class) -> Option<ClassSynthesizedFields> {
        let metadata = self.get_metadata_for_class(cls);

        let base_type = metadata.bases_with_metadata();
        let is_new_type = metadata.is_new_type();

        if is_new_type && base_type.len() == 1 {
            let (base_class, _) = &base_type[0];
            Some(ClassSynthesizedFields::new(smallmap! {
                dunder::NEW => self.get_new_type_new(cls, base_class.clone()),
                dunder::INIT => self.get_new_type_init(cls, base_class.clone()),
            }))
        } else {
            None
        }
    }
}
