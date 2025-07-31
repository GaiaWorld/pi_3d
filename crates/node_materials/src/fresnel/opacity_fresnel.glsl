float opacityFresnel(
    const float absNV
) {
	float x = matParam.opacityFresnelLeft;
	float y = matParam.opacityFresnelRight;

	float opacityFresnelTerm = computeFresnelTerm(absNV, matParam.opacityFresnelParam.x, matParam.opacityFresnelParam.y);
	
	return x * (1.0 - opacityFresnelTerm) + opacityFresnelTerm * y;
}
