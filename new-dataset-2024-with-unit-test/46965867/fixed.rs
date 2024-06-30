struct Material {}

pub struct Sphere<'a> {
    material: &'a Material,
}

pub trait AnySceneObject {}

impl<'a> AnySceneObject for Sphere<'a> {}

pub struct Scene<'a> {
    objects: Vec<Box<AnySceneObject + 'a>>,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let material = Material {};
        let boxed_sphere: Box<dyn AnySceneObject> = Box::new(Sphere {
            material: &material,
        });
        let scene = Scene {
            objects: vec![boxed_sphere],
        };
        assert_eq!(scene.objects.len(), 1);
    }
}
