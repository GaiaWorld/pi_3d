

float noise( vec2 n )
{
	return fract(sin(dot(n.xy, vec2(12.9898, 78.233)))* 43758.5453);
}


float n1noise( vec2 n )
{
	float t = fract(100.0);
	float nrnd0 = noise( n + 0.07*t );
	return nrnd0;
}
