float opacityFresnel(
    float absNV,
) {
	float x = opacityFresnelLeft;
	float y = opacityFresnelRight;

	float opacityFresnelTerm = computeFresnelTerm(absNV, opacityFresnelParam.x, opacityFresnelParam.y);
	
	return x * (1.0 - opacityFresnelTerm) + opacityFresnelTerm * y;
}
