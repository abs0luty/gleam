use super::*;
use crate::type_::{FieldMap, HasType};

pub type TypedConstant = Constant<TypeId, EcoString>;
pub type UntypedConstant = Constant<(), ()>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant<T, RecordTag> {
    Int {
        location: SrcSpan,
        value: EcoString,
    },

    Float {
        location: SrcSpan,
        value: EcoString,
    },

    String {
        location: SrcSpan,
        value: EcoString,
    },

    Tuple {
        location: SrcSpan,
        elements: Vec<Self>,
    },

    List {
        location: SrcSpan,
        elements: Vec<Self>,
        typ: T,
    },

    Record {
        location: SrcSpan,
        module: Option<EcoString>,
        name: EcoString,
        args: Vec<CallArg<Self>>,
        tag: RecordTag,
        typ: T,
        field_map: Option<FieldMap>,
    },

    BitArray {
        location: SrcSpan,
        segments: Vec<BitArraySegment<Self, T>>,
    },

    Var {
        location: SrcSpan,
        module: Option<EcoString>,
        name: EcoString,
        constructor: Option<Box<ValueConstructor>>,
        typ: T,
    },
}

impl TypedConstant {
    pub fn type_(&self) -> TypeId {
        match self {
            Constant::Int { .. } => crate::type_::int(),
            Constant::Float { .. } => crate::type_::float(),
            Constant::String { .. } => crate::type_::string(),
            Constant::BitArray { .. } => crate::type_::bits(),
            Constant::Tuple { elements, .. } => {
                crate::type_::tuple(elements.iter().map(|e| e.type_()).collect())
            }
            Constant::List { typ, .. }
            | Constant::Record { typ, .. }
            | Constant::Var { typ, .. } => typ.clone(),
        }
    }
}

impl HasType for TypedConstant {
    fn type_(&self) -> TypeId {
        self.type_()
    }
}

impl<A, B> Constant<A, B> {
    pub fn location(&self) -> SrcSpan {
        match self {
            Constant::Int { location, .. }
            | Constant::List { location, .. }
            | Constant::Float { location, .. }
            | Constant::Tuple { location, .. }
            | Constant::String { location, .. }
            | Constant::Record { location, .. }
            | Constant::BitArray { location, .. }
            | Constant::Var { location, .. } => *location,
        }
    }

    pub fn is_simple(&self) -> bool {
        matches!(
            self,
            Self::Int { .. } | Self::Float { .. } | Self::String { .. }
        )
    }
}

impl<A, B> HasLocation for Constant<A, B> {
    fn location(&self) -> SrcSpan {
        self.location()
    }
}

impl<A, B> crate::bit_array::GetLiteralValue for Constant<A, B> {
    fn as_int_literal(&self) -> Option<i64> {
        if let Constant::Int { value, .. } = self {
            if let Ok(val) = value.parse::<i64>() {
                return Some(val);
            }
        }
        None
    }
}
