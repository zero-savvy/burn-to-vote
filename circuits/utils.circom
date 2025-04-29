pragma circom 2.0.0;
include "circomlib/circuits/comparators.circom";
include "circomlib/circuits/mux1.circom";

include "circomlib/circuits/bitify.circom";

template VarShiftLeft(n, nBits) {
    signal input in[n]; // x
    signal input shift; // k
    
    signal output out[n]; // y

    component n2b = Num2Bits(nBits);
    n2b.in <== shift;

    signal tmp[nBits][n];
    for (var j = 0; j < nBits; j++) {
        for (var i = 0; i < n; i++) {
            var offset = (i + (1 << j)) % n;
            // Shift left by 2^j indices if bit is 1
            if (j == 0) {
                tmp[j][i] <== n2b.out[j] * (in[offset] - in[i]) + in[i];
            } else {
                tmp[j][i] <== n2b.out[j] * (tmp[j-1][offset] - tmp[j-1][i]) + tmp[j-1][i];
            }
        }
    }
    
    // Return last row
    for (var i = 0; i < n; i++) {
        out[i] <== tmp[nBits - 1][i];
    }
}
template SubArray(nIn, maxSelect, nInBits) {
    signal input in[nIn];
    signal input start;
    signal input end;

    signal output out[maxSelect];
    signal output outLen;

    
    component lt1 = LessEqThan(nInBits);
    lt1.in[0] <== start;
    lt1.in[1] <== end;
    lt1.out === 1;

    component lt2 = LessEqThan(nInBits);
    lt2.in[0] <== end;
    lt2.in[1] <== nIn;
    lt2.out === 1;

    component lt3 = LessEqThan(nInBits);
    lt3.in[0] <== end - start;
    lt3.in[1] <== maxSelect;
    lt3.out === 1;

    outLen <== end - start;

    component n2b = Num2Bits(nInBits);
    n2b.in <== start;

    signal shifts[nInBits][nIn];
    for (var idx = 0; idx < nInBits; idx++) {
        for (var j = 0; j < nIn; j++) {
            if (idx == 0) {
	        var tempIdx = (j + (1 << idx)) % nIn;
                shifts[idx][j] <== n2b.out[idx] * (in[tempIdx] - in[j]) + in[j];
            } else {
	        var prevIdx = idx - 1;
	        var tempIdx = (j + (1 << idx)) % nIn;
                shifts[idx][j] <== n2b.out[idx] * (shifts[prevIdx][tempIdx] - shifts[prevIdx][j]) + shifts[prevIdx][j];            
            }
        }
    }

    for (var idx = 0; idx < maxSelect; idx++) {
        out[idx] <== shifts[nInBits - 1][idx];
    }

}

template BytesToNum(N) {
    signal input bytes[N];   
    signal output num;    
    var pow = 1;
    var total = 0;
    for (var i = 0; i < N; i++) {
        total += pow * bytes[N - 1 - i];  
        pow = pow * 256;         
    }

    num <== total;
}

template PaddedBytesToNum(N) {
    signal input bytes[N];  
    signal input realLen; 
    signal output num;    

    signal pow256[N];
    signal isPowerValid[N];
    component lennCheck[N];

    pow256[0] <== 1;
    for (var i = 1; i < N; i++) {
        lennCheck[i] = LessEqThan(10);
        lennCheck[i].in[0] <== N - i ;
        lennCheck[i].in[1] <== realLen - 1;
        isPowerValid[i] <== lennCheck[i].out * 255 + 1;
        pow256[i] <== pow256[i-1] * isPowerValid[i];
    }


    var paddingCount = N - realLen;
    signal lastIndex;
    lastIndex <== N - paddingCount - 1;
    // var pow = 1;
    var total[N];
    signal weight[N];
    component lenCheck[N];
    signal isLenValid[N];
    // signal isPowerValid[N];
    for (var i = 0; i < N; i++) {
        lenCheck[i] = LessEqThan(10);
        lenCheck[i].in[0] <== N - 1 - i ;
        lenCheck[i].in[1] <== realLen - 1;
        weight[i] <== pow256[i] * bytes[N - 1 - i ];
        isLenValid[i] <== weight[i] * lenCheck[i].out;

        if (i ==0){
            total[i] = isLenValid[i];
        }else{
            total[i] = total[i-1] + isLenValid[i];  
        }
    }

    num <== total[N-1];
}


// aggregated sum verification method
template IsSubarray(n, m) {
    signal input base[n];
    signal input sub[m];
    signal output out;
    var isLenValid = 1;
    if (n < m){
        isLenValid = 0;
    }


    var sub_sum = 0;
    for (var j=0; j<m; j++){
        sub_sum = (sub[j] * (2 ** j)) + sub_sum;
    }
    signal isValid[n-m+1];
    component isSumEq[n-m+1];
    for (var i=0; i<n-m+1; i++){
        var sum = 0 ;
        for (var j=0; j<m; j++){
            sum = base[i+j] * (2 ** j) + sum;

        }
        isSumEq[i] = IsEqual();
        isSumEq[i].in[0] <== sum;
        isSumEq[i].in[1] <== sub_sum;
        if(i == 0) {
            isValid[i] <== isSumEq[i].out;
        } else {
            isValid[i] <== isValid[i-1] + isSumEq[i].out;
        }

    } 

    component isOne = GreaterEqThan(10);
    isOne.in[0] <== isValid[n-m];
    isOne.in[1] <== 1;
    out <== isOne.out ;


}

// aggregated sum verification method
// chekc if a padded subarray exists in an array 
template IsPaddedSubarray(baseLen, subLen) {
    signal input base[baseLen];
    signal input sub[subLen];
    signal input subRealLen;
    signal output out;

    // input checks
    var isLenValid = 1;
    if (baseLen <= subLen){
        isLenValid = 0;
    }
    isLenValid === 1;

    component issubRealLenValid = LessEqThan(10);
    issubRealLenValid.in[0] <== subRealLen;
    issubRealLenValid.in[1] <== subLen;
    issubRealLenValid.out === 1;


    // calculate the aggregated sum of sub array
    // a0*2^0 + a1*2^1 + .... + a(n-1)*2^(n-1)
    var sub_sum = 0;
    component isSubIndexValid[subLen];
    signal subWeight[subLen];

    for (var j=0; j<subLen; j++){
        isSubIndexValid[j] = LessThan(10);
        isSubIndexValid[j].in[0] <== j;
        isSubIndexValid[j].in[1] <== subRealLen;
        subWeight[j] <== (sub[j] * (2 ** j)) * isSubIndexValid[j].out;
        sub_sum = subWeight[j] + sub_sum;

    }


    // calculate the aggregated sum of sub arrays in the main array 
    // a0*2^0 + a1*2^1 + .... + a(n-1)*2^(n-1)

    signal aggregatedSums[baseLen-subLen+1];
    component isIndexValid[baseLen-subLen+1][subLen];
    signal weigth[baseLen-subLen+1][subLen];

    component isSumEq[baseLen-subLen+1];
    for (var i=0; i<baseLen-subLen+1; i++){
        var sum = 0 ;
        for (var j=0; j<subLen; j++){
            isIndexValid[i][j] = LessThan(10);
            isIndexValid[i][j].in[0] <== j;
            isIndexValid[i][j].in[1] <== subRealLen;
            weigth[i][j] <== (base[i+j] * (2 ** j)) * isIndexValid[i][j].out;
            sum = weigth[i][j]  + sum;

        }

        isSumEq[i] = IsEqual();
        isSumEq[i].in[0] <== sum;
        isSumEq[i].in[1] <== sub_sum;

        // add each level with level above(to get the aggregated final sum check at the last index)
        if(i == 0) {
            aggregatedSums[i] <== isSumEq[i].out ;
        } else {
            aggregatedSums[i] <== (aggregatedSums[i-1]) + isSumEq[i].out;

        }

    } 

    // aggregatedSums[baseLen-subLen] is the number of times the subarray apperard in the base array 
    component isOne = GreaterEqThan(10);
    isOne.in[0] <== aggregatedSums[baseLen-subLen];
    isOne.in[1] <== 1;
    out <== isOne.out ;


}

template HexToBytes(hexLen, bytesLen){
    signal input hexArray[hexLen];
    signal output out[bytesLen];

    hexLen/2 === bytesLen;

    var j = 0;
    for (var i=0; i< hexLen; i+=2){
        out[j] <== hexArray[i] * 16 + hexArray[i+1];
        j = j +1;
    }

}

template get_branch_nibbles(n, m){
    signal input branch[n];
    signal input branch_item_len[m];
    signal input nibble_index;

    signal output out;

    // var sum = 0;
    component nibble_len_check[m];
    signal sums[m];
    for (var i=0; i< m; i++){
        nibble_len_check[i] = LessThan(10);
        nibble_len_check[i].in[0] <== i;
        nibble_len_check[i].in[1] <== nibble_index;
        if (i == 0) {
            sums[i] <== branch_item_len[i] * nibble_len_check[i].out;
        } else {
            sums[i] <== sums[i-1] + (branch_item_len[i] * nibble_len_check[i].out);
        }
    }
    
    // component len_check = GreaterEqThan(10);
    // len_check.in[0] <== sum;
    // len_check.in[1] <== 55;


    signal rlp_len <== branch[0] * 16 + branch[1];
    // extra one for the actual data rlp prefix
    signal prefix_len <== (rlp_len - 247) + 1 ;
    signal pp <== prefix_len * 2;
    signal dd <== pp + 2;
    // signal tt <== dd + sum;
    // signal prefix_len <== ((rlp_len - 247) + 1 ) * 2;

    log("sum");
    log(sums[m-1]);
    log("rlp_len");
    log(rlp_len);
    log("prefix_len");
    log(prefix_len);
    out <== dd + sums[m-1] ;
}


template HexToDigits() {
    signal input addr; 
    signal output digits[40]; 

    component n2b = Num2Bits(160);
    n2b.in <== addr;

    for (var i = 0; i < 40; i++) {
        var start_bit = 159 - (4 * i);
        var bit3 = n2b.out[start_bit];
        var bit2 = n2b.out[start_bit - 1];
        var bit1 = n2b.out[start_bit - 2];
        var bit0 = n2b.out[start_bit - 3];
        digits[i] <== bit3 * 8 + bit2 * 4 + bit1 * 2 + bit0 * 1;
    }

}
