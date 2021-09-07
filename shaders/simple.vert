#version 450 core

in vec3 position;
vec3 mirrorVector = vec3(-1.0, -1.0, 1.0);
uniform vec3 scaler = vec3(1.0f,1.0f,1.0f);
void main()
{
    gl_Position = vec4(position*scaler, 1.0f);
}