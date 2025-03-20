let hoge = [1, 2, [3, 4]];
let fuga = [100, 200, 300];
let hoge[1] = hoge[2][0] + hoge[2][1] * fuga[1];
let hoge[0] = hoge[1] + fuga[0] - fuga[2] ;
hoge[0]
