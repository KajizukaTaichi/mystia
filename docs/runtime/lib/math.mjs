export class MystiaMathLib {
    constructor() {
        this.functions = {
            e: () => Math.E,
            pi: () => Math.PI,
            abs: (value) => Math.abs(value),
            acos: (value) => Math.acos(value),
            acosh: (value) => Math.acosh(value),
            asin: (value) => Math.asin(value),
            asinh: (value) => Math.asinh(value),
            atan: (value) => Math.atan(value),
            atan2: (value) => Math.atan2(value),
            atanh: (value) => Math.atanh(value),
            cbrt: (value) => Math.cbrt(value),
            ceil: (value) => Math.ceil(value),
            clz32: (value) => Math.clz32(value),
            cos: (value) => Math.cos(value),
            cosh: (value) => Math.cosh(value),
            exp: (value) => Math.exp(value),
            expm1: (value) => Math.expm1(value),
            floor: (value) => Math.floor(value),
            f16round: (value) => Math.f16round(value),
            fround: (value) => Math.fround(value),
            imul: (value1, value2) => Math.imul(value1, value2),
            log: (value) => Math.log(value),
            log10: (value) => Math.log10(value),
            log1p: (value) => Math.log1p(value),
            log2: (value) => Math.log2(value),
            pow: (value1, value2) => Math.pow(value1, value2),
            rad: (value) => value * (Math.PI / 180),
            round: (value) => Math.round(value),
            sign: (value) => Math.sign(value),
            sin: (value) => Math.sin(value),
            sinh: (value) => Math.sinh(value),
            sqrt: (value) => Math.sqrt(value),
            sum_precise: (value) => Math.sumPrecise(value),
            tan: (value) => Math.tan(value),
            tanh: (value) => Math.tanh(value),
            trunc: (value) => Math.trunc(value),
        };
    }
    set_wasm(instance) {
        this.instance = instance;
    }
    bridge() {
        const b = {};
        for (const k of Object.keys(this.functions)) {
            b[k] = (...a) => this.functions[k](...a);
        }
        return b;
    }
}
