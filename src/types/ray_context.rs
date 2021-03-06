use std::{collections::HashSet, hash::Hash, sync::Arc};

use super::*;

#[derive(Clone, Debug)]
struct HashWrapper(ObjectType);

impl PartialEq for HashWrapper {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for HashWrapper {}

impl Hash for HashWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state)
    }
}

impl Into<HashWrapper> for ObjectType {
    fn into(self) -> HashWrapper {
        HashWrapper(self)
    }
}

type ObjectTypeSet = HashSet<HashWrapper>;

#[derive(Debug)]
pub struct RayContext {
    pub refl_limit: i32,
    pub refr_index: f64,
    refr_objs: ObjectTypeSet,
}

impl RayContext {
    pub fn new(refl_limit: i32) -> Self {
        Self::new_from_objs(refl_limit, HashSet::new())
    }

    fn new_from_objs(refl_limit: i32, refr_objs: ObjectTypeSet) -> Self {
        let mut refr_index = 1.0;
        for obj in refr_objs.iter() {
            if let RefractiveType {
                surface_transparency: _,
                index,
            } = obj.0.material().m_type
            {
                refr_index *= index;
            } else {
                panic!("Non-refractive object in the set of refractive objects");
            }
        }

        Self {
            refl_limit,
            refr_index,
            refr_objs,
        }
    }

    pub fn limit_reached(&self) -> bool {
        self.refl_limit == 0
    }

    pub fn reflected_subray_context(&self) -> Self {
        Self {
            refl_limit: self.refl_limit - 1,
            refr_index: self.refr_index,
            refr_objs: self.refr_objs.clone(),
        }
    }

    pub fn refracted_subray_context(&self, obj: ObjectType) -> Self {
        let wrapper = obj.into();
        let mut refr_objs = self.refr_objs.clone();

        if refr_objs.take(&wrapper).is_none() {
            refr_objs.insert(wrapper);
        }
        Self::new_from_objs(self.refl_limit - 1, refr_objs)
    }
}
