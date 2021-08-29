#version 430 core

in vec3 position;
vec3 mirrorVector = vec3(-1.0, -1.0, 1.0);
void main()
{
    gl_Position = vec4(position, 1.0f);
}