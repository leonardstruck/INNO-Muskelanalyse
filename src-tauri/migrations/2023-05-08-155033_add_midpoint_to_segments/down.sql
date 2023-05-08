-- This file should undo anything in `up.sql`
ALTER TABLE segments DROP COLUMN measured_midpoint_x;
ALTER TABLE segments DROP COLUMN measured_midpoint_y;