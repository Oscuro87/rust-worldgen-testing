use super::tile::{Soil, Tile};
use crate::wg_utils::point_2d::Point2D;

pub trait Layer<T> {
    fn create_empty(width: i32, height: i32) -> Self;
    fn create(width: i32, height: i32, fill_with: Vec<T>) -> Self;
    fn get_cell_ref_from_point(&self, x: i32, y: i32) -> &T;
    fn get_mutable_cell_ref_from_point(&mut self, x: i32, y: i32) -> &mut T;
    // fn does_cell_exist_at_point(&self, x: i32, y: i32) -> bool;
    fn replace_cell(&mut self, at_position: (i32, i32), with: T) -> ();
}

pub struct TileLayer {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Layer<Tile> for TileLayer {
    fn create(width: i32, height: i32, fill_with: Vec<Tile>) -> Self {
        TileLayer {
            width,
            height,
            tiles: fill_with,
        }
    }

    fn create_empty(width: i32, height: i32) -> Self {
        let generated: Vec<Tile> = std::iter::repeat(Tile::default())
            .take((width * height) as usize)
            .collect();

        TileLayer {
            width,
            height,
            tiles: generated,
        }
    }

    fn get_cell_ref_from_point(&self, x: i32, y: i32) -> &Tile {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        &self.tiles[Point2D::calc_index_from_point(Point2D::create(x, y), self.width) as usize]
    }

    fn get_mutable_cell_ref_from_point(&mut self, x: i32, y: i32) -> &mut Tile {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        &mut self.tiles[Point2D::calc_index_from_point(Point2D::create(x, y), self.width) as usize]
    }

    fn replace_cell(&mut self, at_position: (i32, i32), with: Tile) -> () {
        let index: usize = Point2D::calc_index_from_point(at_position.into(), self.width) as usize;
        self.tiles[index] = with;
    }
}

pub struct SoilLayer {
    width: i32,
    height: i32,
    soils: Vec<Soil>,
}

impl Layer<Soil> for SoilLayer {
    fn create(width: i32, height: i32, fill_with: Vec<Soil>) -> Self {
        SoilLayer {
            width,
            height,
            soils: fill_with,
        }
    }

    fn create_empty(width: i32, height: i32) -> Self {
        let generated: Vec<Soil> = std::iter::repeat(Soil::default())
            .take((width * height) as usize)
            .collect();

        SoilLayer {
            width,
            height,
            soils: generated,
        }
    }

    fn get_cell_ref_from_point(&self, x: i32, y: i32) -> &Soil {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        &self.soils[Point2D::calc_index_from_point(Point2D::create(x, y), self.width) as usize]
    }

    fn get_mutable_cell_ref_from_point(&mut self, x: i32, y: i32) -> &mut Soil {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        &mut self.soils[Point2D::calc_index_from_point(Point2D::create(x, y), self.width) as usize]
    }

    fn replace_cell(&mut self, at_position: (i32, i32), with: Soil) -> () {
        let index: usize = Point2D::calc_index_from_point(at_position.into(), self.width) as usize;
        self.soils[index] = with;
    }
}
