#version 430 core

uniform layout(location = 1) vec3 uColor = vec3(0.9f, 0.2f, 0.1f);
out vec4 color;

void main()
{
    //color = vec4(0.9f, 0.2f, 0.1f, 1.0f);
    color = vec4(uColor[0], uColor[1], uColor[2], 1.0f);
}