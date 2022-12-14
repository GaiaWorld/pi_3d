
mat4 jointMat(int index) {  
    if (index == 0) {
        return u_jointMat0;
    }

    if (index == 1){
        return u_jointMat1;
    }

    if (index == 2){
        return u_jointMat2;
    }

    if (index == 3){
        return u_jointMat3;
    }

    return u_jointMat0;
}

