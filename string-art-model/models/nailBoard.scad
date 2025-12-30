include <../constants/nailBoard.scad>

module nail(position = [0,0], angle = 0) {
    color(nail_colour)
    translate(position)
        rotate([0,0,angle])
            union() {
                // Nail shaft
                translate([0,0,nail_board_thickness])
                    cylinder(h = nail_height, r = nail_radius, $fn=50);
                // Nail head
                translate([0,0,nail_board_thickness + nail_height])
                    cylinder(h = nail_head_height, r = nail_head_radius, $fn=50);
            }
}

module nail_board() {
    color(nail_board_colour)
    difference() {
        cylinder(h = nail_board_thickness, r = nail_board_radius, $fn=100);
        translate([0,0,-1])
            cylinder(h = nail_board_thickness+2, r = nail_board_axel_radius, $fn=100);
    }
}

module nail_board_with_nails() {
    nail_board();
    // Place nails
    for (i = [0 : number_of_nails - 1]) {
        angle = i * 360 / number_of_nails;
        x = (nail_board_radius - nail_board_nail_indent) * cos(angle);
        y = (nail_board_radius - nail_board_nail_indent) * sin(angle);
        nail(position = [x, y], angle = angle + 90);
    }
}