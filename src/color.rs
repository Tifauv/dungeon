pub fn to_linear_rgba(color: [f32; 4]) -> [f32; 4] {
	[
		f32::powf((color[0] / 255.0 + 0.055) / 1.055, 2.4), // R
		f32::powf((color[1] / 255.0 + 0.055) / 1.055, 2.4), // G
		f32::powf((color[2] / 255.0 + 0.055) / 1.055, 2.4), // B
		color[3]                                            // A
	]
}
