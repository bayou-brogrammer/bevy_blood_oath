use super::*;

#[derive(Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum DistanceAlgorithm {
    Pythagoras,
    Manhattan,
    Chebyshev,
}

pub struct VoronoiCellBuilder {
    n_seeds: usize,
    distance_algorithm: DistanceAlgorithm,
}

impl InitialMapBuilder for VoronoiCellBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, build_data: &mut BuilderMap) { self.build(build_data); }
}

impl VoronoiCellBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<VoronoiCellBuilder> {
        Box::new(VoronoiCellBuilder { n_seeds: 64, distance_algorithm: DistanceAlgorithm::Pythagoras })
    }

    #[allow(dead_code)]
    pub fn pythagoras() -> Box<VoronoiCellBuilder> {
        Box::new(VoronoiCellBuilder { n_seeds: 64, distance_algorithm: DistanceAlgorithm::Pythagoras })
    }

    #[allow(dead_code)]
    pub fn manhattan() -> Box<VoronoiCellBuilder> {
        Box::new(VoronoiCellBuilder { n_seeds: 64, distance_algorithm: DistanceAlgorithm::Manhattan })
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self, build_data: &mut BuilderMap) {
        // Make a Voronoi diagram. We'll do this the hard way to learn about the technique!
        let mut voronoi_seeds: Vec<(usize, Point)> = Vec::new();

        while voronoi_seeds.len() < self.n_seeds {
            let vx = crate::rng::roll_dice(1, build_data.map.width - 1);
            let vy = crate::rng::roll_dice(1, build_data.map.height - 1);
            let vidx = build_data.map.xy_idx(vx, vy);
            let candidate = (vidx, Point::new(vx, vy));
            if !voronoi_seeds.contains(&candidate) {
                voronoi_seeds.push(candidate);
            }
        }

        let mut voronoi_distance = vec![(0, 0.0f32); self.n_seeds];
        let mut voronoi_membership: Vec<i32> =
            vec![0; build_data.map.width as usize * build_data.map.height as usize];
        for (i, vid) in voronoi_membership.iter_mut().enumerate() {
            let pt = build_data.map.index_to_point2d(i);

            for (seed, pos) in voronoi_seeds.iter().enumerate() {
                let distance = match self.distance_algorithm {
                    DistanceAlgorithm::Manhattan => DistanceAlg::Manhattan.distance2d(pt, pos.1),
                    DistanceAlgorithm::Chebyshev => DistanceAlg::Chebyshev.distance2d(pt, pos.1),
                    DistanceAlgorithm::Pythagoras => DistanceAlg::PythagorasSquared.distance2d(pt, pos.1),
                };
                voronoi_distance[seed] = (seed, distance);
            }

            voronoi_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            *vid = voronoi_distance[0].0 as i32;
        }

        for y in 1..build_data.map.height - 1 {
            for x in 1..build_data.map.width - 1 {
                let mut neighbors = 0;
                let my_idx = build_data.map.xy_idx(x, y);
                let my_seed = voronoi_membership[my_idx];
                if voronoi_membership[build_data.map.xy_idx(x - 1, y)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[build_data.map.xy_idx(x + 1, y)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[build_data.map.xy_idx(x, y - 1)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[build_data.map.xy_idx(x, y + 1)] != my_seed {
                    neighbors += 1;
                }

                if neighbors < 2 {
                    build_data.map.tiles[my_idx] = GameTile::floor()
                }
            }
            build_data.take_snapshot();
        }
    }
}
