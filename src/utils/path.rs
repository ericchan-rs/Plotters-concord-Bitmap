// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

type PathSimplifierPointInner = [i32; 2];
type PathSimplifierPointOuter = [f64; 2];

enum PathSimplifierGroup {
    None,
    X(i32),
    Y(i32),
}

pub(crate) struct PathSimplifier<I: Iterator<Item = PathSimplifierPointInner>> {
    source_points: I,
    current_group: PathSimplifierGroup,
    last_point: Option<PathSimplifierPointInner>,
}

impl<I: Iterator<Item = PathSimplifierPointInner>> PathSimplifier<I> {
    pub(crate) fn from(source_points: I) -> Self {
        Self {
            source_points,
            current_group: PathSimplifierGroup::None,
            last_point: None,
        }
    }
}

impl<I: Iterator<Item = PathSimplifierPointInner>> Iterator for PathSimplifier<I> {
    type Item = PathSimplifierPointOuter;

    fn next(&mut self) -> Option<Self::Item> {
        // Branch to source points iterator (exhaust next group)
        while let Some(point) = self.source_points.next() {
            // Backtrack in points
            if let Some(point_before) = self.last_point {
                // Retain current point as 'last point'
                self.last_point = Some(point);

                // De-duplicate points
                if point_before != point {
                    let mut do_yield = false;

                    match self.current_group {
                        PathSimplifierGroup::None => {
                            if point_before[0] == point[0] {
                                self.current_group = PathSimplifierGroup::X(point_before[0]);
                            } else if point_before[1] == point[1] {
                                self.current_group = PathSimplifierGroup::Y(point_before[1]);
                            }

                            // Yield start-of-group or isolated point
                            do_yield = true;
                        }
                        PathSimplifierGroup::X(opener_x) => {
                            // Close current X group? (using 'before' point)
                            if point[0] != opener_x {
                                self.current_group = PathSimplifierGroup::None;

                                // Yield end-of-group point
                                do_yield = true;
                            }
                        }
                        PathSimplifierGroup::Y(opener_y) => {
                            // Close current Y group? (using 'before' point)
                            if point[1] != opener_y {
                                self.current_group = PathSimplifierGroup::None;

                                // Yield end-of-group point
                                do_yield = true;
                            }
                        }
                    }

                    if do_yield {
                        return Some([point_before[0] as _, point_before[1] as _]);
                    }
                }
            } else {
                // Retain first point as 'last point'
                self.last_point = Some(point);
            }
        }

        // End of the source points iterator, close path? (this yields)
        if let Some(last_point) = self.last_point {
            self.last_point = None;

            return Some([last_point[0] as _, last_point[1] as _]);
        }

        // Done painting all path points
        None
    }
}
