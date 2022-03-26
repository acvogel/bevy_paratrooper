#![allow(missing_docs)]

//! Type conversions between bevy and nalgebra (used by rapier's API)
//!
//! Provides the [`IntoBevy`](IntoBevy) and [`IntoRapier`](IntoRapier)
//! with implementations for bevy and rapier types

use bevy::math::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_rapier2d::prelude::nalgebra::{
    self, Point2, Point3, Quaternion, UnitComplex, UnitQuaternion, Vector2, Vector3,
};

pub trait IntoBevy<T> {
    #[must_use]
    fn into_bevy(self) -> T;
}

pub trait IntoRapier<T> {
    #[must_use]
    fn into_rapier(self) -> T;
}

impl IntoBevy<Vec3> for Vector2<f32> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

impl IntoBevy<Vec3> for Vector3<f32> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl IntoBevy<Vec3> for Translation<f32> {
    fn into_bevy(self) -> Vec3 {
        self.vector.into_bevy()
    }
}

impl IntoBevy<Quat> for UnitComplex<f32> {
    fn into_bevy(self) -> Quat {
        Quat::from_axis_angle(Vec3::Z, self.angle())
    }
}

impl IntoBevy<Quat> for UnitQuaternion<f32> {
    fn into_bevy(self) -> Quat {
        Quat::from_xyzw(self.i, self.j, self.k, self.w)
    }
}

impl IntoBevy<(Vec3, Quat)> for Isometry<f32> {
    fn into_bevy(self) -> (Vec3, Quat) {
        (self.translation.into_bevy(), self.rotation.into_bevy())
    }
}

impl IntoRapier<Vector2<f32>> for Vec2 {
    fn into_rapier(self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
}

impl IntoRapier<Vector2<f32>> for Vec3 {
    fn into_rapier(self) -> Vector2<f32> {
        self.truncate().into_rapier()
    }
}

impl IntoRapier<Vector3<f32>> for Vec3 {
    fn into_rapier(self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl IntoRapier<Point2<f32>> for Vec2 {
    fn into_rapier(self) -> Point2<f32> {
        Point2 {
            coords: self.into_rapier(),
        }
    }
}

impl IntoRapier<Point2<f32>> for Vec3 {
    fn into_rapier(self) -> Point2<f32> {
        Point2 {
            coords: self.into_rapier(),
        }
    }
}

impl IntoRapier<Point3<f32>> for Vec3 {
    fn into_rapier(self) -> Point3<f32> {
        Point3 {
            coords: self.into_rapier(),
        }
    }
}

impl IntoRapier<Vec<Point2<f32>>> for &[Vec3] {
    fn into_rapier(self) -> Vec<Point2<f32>> {
        self.iter().copied().map(IntoRapier::into_rapier).collect()
    }
}

impl IntoRapier<Vec<Point3<f32>>> for &[Vec3] {
    fn into_rapier(self) -> Vec<Point3<f32>> {
        self.iter().copied().map(IntoRapier::into_rapier).collect()
    }
}

impl IntoBevy<Vec2> for Point2<f32> {
    fn into_bevy(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl IntoBevy<Vec3> for Point3<f32> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl IntoBevy<Vec<Vec2>> for &[Point2<f32>] {
    fn into_bevy(self) -> Vec<Vec2> {
        self.iter().copied().map(IntoBevy::into_bevy).collect()
    }
}

impl IntoRapier<Translation<f32>> for Vec3 {
    fn into_rapier(self) -> Translation<f32> {
        <Vec3 as IntoRapier<Vector<f32>>>::into_rapier(self).into()
    }
}

impl IntoRapier<UnitComplex<f32>> for Quat {
    fn into_rapier(self) -> UnitComplex<f32> {
        let (axis, angle) = self.to_axis_angle();
        nalgebra::UnitComplex::new(if axis.z > 0.0 { angle } else { -angle })
    }
}

impl IntoRapier<UnitQuaternion<f32>> for Quat {
    fn into_rapier(self) -> UnitQuaternion<f32> {
        UnitQuaternion::new_normalize(Quaternion::new(self.w, self.x, self.y, self.z))
    }
}

impl IntoRapier<Isometry<f32>> for (Vec3, Quat) {
    fn into_rapier(self) -> Isometry<f32> {
        Isometry::from_parts(self.0.into_rapier(), self.1.into_rapier())
    }
}
