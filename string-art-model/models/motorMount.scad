include <../constants/motorMount.scad>

module motor_mount() {
    color(motor_mount_colour)
    difference() {
        cube([motor_mount_width, motor_mount_height, motor_mount_thickness], center=true);

        // Axle hole
        translate([motor_mount_axel_offset_x, motor_mount_axel_offset_y, -motor_mount_thickness]) {
            cylinder(h = motor_mount_thickness*2, r = motor_mount_axel_radius, $fn=100);
        }

    }
    
    // Fabric pole
    color(motor_mount_colour)
    translate([motor_mount_fabric_pole_offset_x, motor_mount_fabric_pole_offset_y, motor_mount_thickness/2]) {
        cylinder(h = motor_mount_fabric_pole_height, r = motor_mount_fabric_pole_radius, $fn=100);
    }
}