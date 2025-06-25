import { read } from "../ffi.mjs";

export class MystiaMathLib {
    constructor() {
        this.functions = {
            e: () => Math.E,
            pi: () => Math.PI,
            abs: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.abs(v);
            },
            acos: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.acos(v);
            },
            acosh: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.acosh(v);
            },
            asin: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.asin(v);
            },
            asinh: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.asinh(v);
            },
            atan: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.atan(v);
            },
            atan2: (value1, value1_typ, value2, value2_typ) => {
                const v1 = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value1_typ).toString()})`,
                    ),
                    value1,
                );
                const v2 = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value2_typ).toString()})`,
                    ),
                    value2,
                );
                return Math.atan2(v1, v2);
            },
            atanh: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.atanh(v);
            },
            cbrt: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.cbrt(v);
            },
            ceil: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.ceil(v);
            },
            clz32: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.clz32(v);
            },
            cos: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.cos(v);
            },
            cosh: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.cosh(v);
            },
            exp: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.exp(v);
            },
            expm1: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.expm1(v);
            },
            floor: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.floor(v);
            },
            f16round: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.f16round(v);
            },
            fround: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.fround(v);
            },
            imul: (value1, value1_typ, value2, value2_typ) => {
                const v1 = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value1_typ).toString()})`,
                    ),
                    value1,
                );
                const v2 = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value2_typ).toString()})`,
                    ),
                    value2,
                );
                return Math.imul(v1, v2);
            },
            log: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.log(v);
            },
            log10: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.log10(v);
            },
            log1p: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.log1p(v);
            },
            log2: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.log2(v);
            },
            pow: (value1, value1_typ, value2, value2_typ) => {
                const v1 = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value1_typ).toString()})`,
                    ),
                    value1,
                );
                const v2 = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value2_typ).toString()})`,
                    ),
                    value2,
                );
                return Math.pow(v1, v2);
            },
            rad: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return v * (Math.PI / 180);
            },
            round: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.round(v);
            },
            sign: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.sign(v);
            },
            sin: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.sin(v);
            },
            sinh: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.sinh(v);
            },
            sqrt: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.sqrt(v);
            },
            sum_precise: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.sumPrecise(v);
            },
            tan: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.tan(v);
            },
            tanh: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.tanh(v);
            },
            trunc: (value, value_typ) => {
                const v = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return Math.trunc(v);
            },
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
