uniform mat4 matrix;
uniform mat4 model;

in vec3 pos;

void main()
{
    gl_Position = matrix * model * vec4(pos, 1.0);
}  