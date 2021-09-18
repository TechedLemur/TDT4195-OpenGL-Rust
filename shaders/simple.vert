#version 450 core

in layout(location=0) vec3 position;

in layout(location=1) vec4 color;

out layout(location=2) vec4 outColor;
//vec3 mirrorVector = vec3(-1.0, -1.0, 1.0);
//uniform vec3 scaler = vec3(1.0f,1.0f,1.0f);
void main()
{
    //gl_Position = vec4(position*scaler, 1.0f);
    gl_Position = vec4(position, 1.0f);
    outColor = color;
}