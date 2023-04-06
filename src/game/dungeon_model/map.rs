use super::Coord;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource)]
pub struct Map {
    pub floor_coords: HashSet<Coord>,
    pub player_start: Coord,
    pub enemy_positions: HashSet<Coord>,
    pub book_positions: HashSet<Coord>,
    pub chest_positions: HashSet<Coord>,
    pub food_positions: HashSet<Coord>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            floor_coords: vec![
                (56, 0),
                (57, 0),
                (58, 0),
                (59, 0),
                (60, 0),
                (61, 0),
                (62, 0),
                (63, 0),
                (56, 1),
                (57, 1),
                (58, 1),
                (59, 1),
                (60, 1),
                (61, 1),
                (62, 1),
                (63, 1),
                (56, 2),
                (57, 2),
                (58, 2),
                (59, 2),
                (60, 2),
                (61, 2),
                (62, 2),
                (63, 2),
                (55, 3),
                (56, 3),
                (57, 3),
                (58, 3),
                (59, 3),
                (60, 3),
                (61, 3),
                (62, 3),
                (63, 3),
                (55, 4),
                (56, 4),
                (57, 4),
                (58, 4),
                (59, 4),
                (60, 4),
                (61, 4),
                (62, 4),
                (63, 4),
                (54, 5),
                (55, 5),
                (56, 5),
                (57, 5),
                (58, 5),
                (59, 5),
                (60, 5),
                (61, 5),
                (62, 5),
                (63, 5),
                (53, 6),
                (54, 6),
                (55, 6),
                (56, 6),
                (57, 6),
                (58, 6),
                (59, 6),
                (60, 6),
                (61, 6),
                (53, 7),
                (54, 7),
                (55, 7),
                (56, 7),
                (57, 7),
                (58, 7),
                (59, 7),
                (53, 8),
                (54, 8),
                (55, 8),
                (56, 8),
                (52, 9),
                (53, 9),
                (54, 9),
                (55, 9),
                (51, 10),
                (52, 10),
                (53, 10),
                (54, 10),
                (39, 11),
                (40, 11),
                (41, 11),
                (42, 11),
                (43, 11),
                (44, 11),
                (45, 11),
                (46, 11),
                (50, 11),
                (51, 11),
                (52, 11),
                (53, 11),
                (36, 12),
                (37, 12),
                (38, 12),
                (39, 12),
                (40, 12),
                (41, 12),
                (42, 12),
                (43, 12),
                (44, 12),
                (45, 12),
                (46, 12),
                (47, 12),
                (48, 12),
                (49, 12),
                (50, 12),
                (51, 12),
                (52, 12),
                (53, 12),
                (34, 13),
                (35, 13),
                (36, 13),
                (37, 13),
                (38, 13),
                (39, 13),
                (40, 13),
                (41, 13),
                (42, 13),
                (43, 13),
                (44, 13),
                (45, 13),
                (46, 13),
                (47, 13),
                (48, 13),
                (49, 13),
                (50, 13),
                (33, 14),
                (34, 14),
                (35, 14),
                (36, 14),
                (44, 14),
                (45, 14),
                (46, 14),
                (47, 14),
                (50, 14),
                (32, 15),
                (33, 15),
                (34, 15),
                (35, 15),
                (43, 15),
                (44, 15),
                (50, 15),
                (30, 16),
                (31, 16),
                (32, 16),
                (33, 16),
                (34, 16),
                (43, 16),
                (44, 16),
                (49, 16),
                (50, 16),
                (51, 16),
                (30, 17),
                (31, 17),
                (42, 17),
                (43, 17),
                (44, 17),
                (49, 17),
                (50, 17),
                (51, 17),
                (30, 18),
                (31, 18),
                (40, 18),
                (41, 18),
                (42, 18),
                (43, 18),
                (44, 18),
                (49, 18),
                (50, 18),
                (51, 18),
                (30, 19),
                (31, 19),
                (40, 19),
                (41, 19),
                (42, 19),
                (43, 19),
                (50, 19),
                (51, 19),
                (29, 20),
                (30, 20),
                (31, 20),
                (39, 20),
                (40, 20),
                (41, 20),
                (42, 20),
                (50, 20),
                (51, 20),
                (29, 21),
                (30, 21),
                (38, 21),
                (39, 21),
                (40, 21),
                (41, 21),
                (42, 21),
                (51, 21),
                (28, 22),
                (29, 22),
                (30, 22),
                (37, 22),
                (38, 22),
                (39, 22),
                (40, 22),
                (41, 22),
                (50, 22),
                (51, 22),
                (28, 23),
                (29, 23),
                (30, 23),
                (31, 23),
                (36, 23),
                (37, 23),
                (38, 23),
                (39, 23),
                (40, 23),
                (50, 23),
                (29, 24),
                (30, 24),
                (31, 24),
                (34, 24),
                (35, 24),
                (36, 24),
                (37, 24),
                (38, 24),
                (39, 24),
                (50, 24),
                (51, 24),
                (29, 25),
                (30, 25),
                (31, 25),
                (32, 25),
                (33, 25),
                (34, 25),
                (35, 25),
                (36, 25),
                (37, 25),
                (51, 25),
                (29, 26),
                (30, 26),
                (31, 26),
                (32, 26),
                (50, 26),
                (51, 26),
                (29, 27),
                (30, 27),
                (31, 27),
                (32, 27),
                (50, 27),
                (29, 28),
                (30, 28),
                (31, 28),
                (32, 28),
                (50, 28),
                (51, 28),
                (30, 29),
                (31, 29),
                (32, 29),
                (33, 29),
                (51, 29),
                (32, 30),
                (33, 30),
                (34, 30),
                (35, 30),
                (50, 30),
                (51, 30),
                (27, 31),
                (33, 31),
                (34, 31),
                (35, 31),
                (36, 31),
                (50, 31),
                (23, 32),
                (24, 32),
                (25, 32),
                (26, 32),
                (27, 32),
                (33, 32),
                (34, 32),
                (35, 32),
                (43, 32),
                (44, 32),
                (45, 32),
                (46, 32),
                (47, 32),
                (48, 32),
                (49, 32),
                (50, 32),
                (51, 32),
                (22, 33),
                (23, 33),
                (24, 33),
                (25, 33),
                (26, 33),
                (27, 33),
                (34, 33),
                (35, 33),
                (36, 33),
                (41, 33),
                (42, 33),
                (43, 33),
                (44, 33),
                (45, 33),
                (46, 33),
                (21, 34),
                (22, 34),
                (23, 34),
                (34, 34),
                (35, 34),
                (36, 34),
                (41, 34),
                (42, 34),
                (43, 34),
                (20, 35),
                (21, 35),
                (22, 35),
                (34, 35),
                (35, 35),
                (36, 35),
                (40, 35),
                (41, 35),
                (42, 35),
                (19, 36),
                (20, 36),
                (21, 36),
                (34, 36),
                (35, 36),
                (40, 36),
                (41, 36),
                (18, 37),
                (19, 37),
                (20, 37),
                (21, 37),
                (33, 37),
                (34, 37),
                (35, 37),
                (40, 37),
                (41, 37),
                (15, 38),
                (16, 38),
                (18, 38),
                (19, 38),
                (20, 38),
                (30, 38),
                (31, 38),
                (32, 38),
                (33, 38),
                (34, 38),
                (35, 38),
                (36, 38),
                (40, 38),
                (41, 38),
                (14, 39),
                (15, 39),
                (16, 39),
                (17, 39),
                (18, 39),
                (19, 39),
                (20, 39),
                (28, 39),
                (29, 39),
                (30, 39),
                (31, 39),
                (32, 39),
                (33, 39),
                (35, 39),
                (36, 39),
                (40, 39),
                (41, 39),
                (13, 40),
                (14, 40),
                (15, 40),
                (16, 40),
                (17, 40),
                (18, 40),
                (19, 40),
                (20, 40),
                (27, 40),
                (28, 40),
                (29, 40),
                (30, 40),
                (31, 40),
                (35, 40),
                (40, 40),
                (41, 40),
                (13, 41),
                (14, 41),
                (17, 41),
                (18, 41),
                (19, 41),
                (20, 41),
                (27, 41),
                (28, 41),
                (29, 41),
                (35, 41),
                (40, 41),
                (41, 41),
                (13, 42),
                (17, 42),
                (18, 42),
                (19, 42),
                (20, 42),
                (21, 42),
                (22, 42),
                (27, 42),
                (28, 42),
                (35, 42),
                (36, 42),
                (39, 42),
                (40, 42),
                (41, 42),
                (11, 43),
                (12, 43),
                (13, 43),
                (18, 43),
                (19, 43),
                (20, 43),
                (21, 43),
                (22, 43),
                (23, 43),
                (24, 43),
                (25, 43),
                (26, 43),
                (27, 43),
                (34, 43),
                (35, 43),
                (36, 43),
                (37, 43),
                (38, 43),
                (39, 43),
                (40, 43),
                (11, 44),
                (12, 44),
                (13, 44),
                (17, 44),
                (18, 44),
                (19, 44),
                (20, 44),
                (21, 44),
                (22, 44),
                (23, 44),
                (24, 44),
                (25, 44),
                (26, 44),
                (27, 44),
                (34, 44),
                (35, 44),
                (36, 44),
                (37, 44),
                (38, 44),
                (39, 44),
                (12, 45),
                (13, 45),
                (16, 45),
                (17, 45),
                (18, 45),
                (19, 45),
                (20, 45),
                (23, 45),
                (24, 45),
                (25, 45),
                (26, 45),
                (27, 45),
                (28, 45),
                (33, 45),
                (34, 45),
                (35, 45),
                (36, 45),
                (12, 46),
                (13, 46),
                (16, 46),
                (17, 46),
                (26, 46),
                (27, 46),
                (28, 46),
                (29, 46),
                (30, 46),
                (31, 46),
                (32, 46),
                (33, 46),
                (34, 46),
                (35, 46),
                (36, 46),
                (12, 47),
                (13, 47),
                (16, 47),
                (17, 47),
                (28, 47),
                (32, 47),
                (33, 47),
                (34, 47),
                (35, 47),
                (6, 48),
                (7, 48),
                (12, 48),
                (15, 48),
                (16, 48),
                (20, 48),
                (21, 48),
                (22, 48),
                (32, 48),
                (33, 48),
                (34, 48),
                (5, 49),
                (6, 49),
                (7, 49),
                (8, 49),
                (11, 49),
                (12, 49),
                (14, 49),
                (15, 49),
                (16, 49),
                (20, 49),
                (21, 49),
                (22, 49),
                (23, 49),
                (24, 49),
                (32, 49),
                (33, 49),
                (7, 50),
                (8, 50),
                (10, 50),
                (11, 50),
                (12, 50),
                (13, 50),
                (14, 50),
                (15, 50),
                (20, 50),
                (21, 50),
                (22, 50),
                (24, 50),
                (32, 50),
                (33, 50),
                (7, 51),
                (8, 51),
                (9, 51),
                (10, 51),
                (11, 51),
                (12, 51),
                (13, 51),
                (14, 51),
                (15, 51),
                (24, 51),
                (27, 51),
                (28, 51),
                (29, 51),
                (32, 51),
                (33, 51),
                (36, 51),
                (7, 52),
                (8, 52),
                (9, 52),
                (10, 52),
                (11, 52),
                (12, 52),
                (13, 52),
                (14, 52),
                (15, 52),
                (24, 52),
                (27, 52),
                (28, 52),
                (29, 52),
                (30, 52),
                (31, 52),
                (32, 52),
                (33, 52),
                (34, 52),
                (35, 52),
                (36, 52),
                (7, 53),
                (8, 53),
                (9, 53),
                (10, 53),
                (11, 53),
                (12, 53),
                (13, 53),
                (14, 53),
                (15, 53),
                (16, 53),
                (17, 53),
                (18, 53),
                (19, 53),
                (20, 53),
                (21, 53),
                (22, 53),
                (23, 53),
                (24, 53),
                (27, 53),
                (28, 53),
                (29, 53),
                (32, 53),
                (33, 53),
                (34, 53),
                (8, 54),
                (9, 54),
                (12, 54),
                (13, 54),
                (14, 54),
                (15, 54),
                (16, 54),
                (17, 54),
                (18, 54),
                (19, 54),
                (32, 54),
                (33, 54),
                (7, 55),
                (8, 55),
                (13, 55),
                (14, 55),
                (15, 55),
                (19, 55),
                (20, 55),
                (32, 55),
                (33, 55),
                (7, 56),
                (8, 56),
                (11, 56),
                (12, 56),
                (13, 56),
                (14, 56),
                (15, 56),
                (20, 56),
                (21, 56),
                (31, 56),
                (32, 56),
                (33, 56),
                (7, 57),
                (8, 57),
                (9, 57),
                (10, 57),
                (11, 57),
                (12, 57),
                (13, 57),
                (14, 57),
                (15, 57),
                (20, 57),
                (21, 57),
                (22, 57),
                (23, 57),
                (30, 57),
                (31, 57),
                (32, 57),
                (6, 58),
                (7, 58),
                (8, 58),
                (9, 58),
                (10, 58),
                (11, 58),
                (12, 58),
                (13, 58),
                (14, 58),
                (15, 58),
                (21, 58),
                (22, 58),
                (23, 58),
                (24, 58),
                (25, 58),
                (28, 58),
                (29, 58),
                (30, 58),
                (31, 58),
                (3, 59),
                (4, 59),
                (5, 59),
                (6, 59),
                (7, 59),
                (8, 59),
                (11, 59),
                (12, 59),
                (13, 59),
                (14, 59),
                (15, 59),
                (16, 59),
                (22, 59),
                (23, 59),
                (24, 59),
                (25, 59),
                (26, 59),
                (27, 59),
                (28, 59),
                (29, 59),
                (2, 60),
                (3, 60),
                (4, 60),
                (5, 60),
                (6, 60),
                (7, 60),
                (13, 60),
                (14, 60),
                (15, 60),
                (16, 60),
                (22, 60),
                (23, 60),
                (24, 60),
                (25, 60),
                (26, 60),
                (27, 60),
                (28, 60),
                (29, 60),
                (30, 60),
                (1, 61),
                (2, 61),
                (3, 61),
                (4, 61),
                (5, 61),
                (28, 61),
                (29, 61),
                (30, 61),
                (1, 62),
                (2, 62),
                (3, 62),
                (4, 62),
            ]
            .iter()
            .map(|(x, y)| Coord(*x, *y))
            .collect(),

            player_start: Coord(3, 61),

            enemy_positions: vec![
                (36, 13),
                (44, 14),
                (31, 16),
                (51, 21),
                (50, 23),
                (33, 25),
                (51, 25),
                (50, 27),
                (51, 29),
                (33, 30),
                (50, 31),
                (21, 34),
                (22, 35),
                (34, 37),
                (35, 41),
                (40, 42),
                (27, 43),
                (17, 45),
                (12, 48),
                (32, 49),
                (33, 49),
                (8, 51),
                (7, 52),
                (31, 52),
                (34, 52),
                (22, 53),
                (19, 55),
                (29, 59),
                (28, 60),
            ]
            .iter()
            .map(|(x, y)| Coord(*x, *y))
            .collect(),

            book_positions: vec![
                (28, 22),
                (51, 22),
                (50, 24),
                (51, 26),
                (50, 28),
                (51, 30),
                (27, 31),
                (36, 31),
                (11, 43),
                (28, 47),
                (5, 49),
                (36, 51),
                (16, 60),
                (30, 61),
            ]
            .iter()
            .map(|(x, y)| Coord(*x, *y))
            .collect(),

            chest_positions: vec![(50, 17), (41, 20), (26, 32), (21, 49), (28, 52)]
                .iter()
                .map(|(x, y)| Coord(*x, *y))
                .collect(),

            food_positions: vec![(33, 15), (42, 34), (25, 44), (24, 59)]
                .iter()
                .map(|(x, y)| Coord(*x, *y))
                .collect(),
        }
    }
}
