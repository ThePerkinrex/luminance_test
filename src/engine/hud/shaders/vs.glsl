// #version 150

// those are our vertex attributes
in ivec2 position;
in uvec2 uv;

// this is the output of the vertex shader (we could have had several ones)
out vec2 v_uv;

uniform ivec2 pos;
uniform float scale;
uniform uvec2 size;
uniform uvec2 tex_size;
uniform float depth;


void main(){
	// Create uv pos from pixel pos & tex size
	v_uv = vec2(float(uv[0])/float(tex_size[0]), float(uv[1])/float(tex_size[1]));

	// mandatory; tell the GPU to use the position vertex attribute to put the vertex in space
	gl_Position = vec4((float(position[0])*scale+float(pos[0]))*2./float(size[0]) - 1., (float(position[1])*scale+float(pos[1]))*2./float(size[1]) - 1.,depth,1.);
}