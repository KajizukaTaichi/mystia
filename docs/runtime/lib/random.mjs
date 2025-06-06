import { write, read } from "../ffi.mjs";

class MersenneTwister {
    constructor(seed) {
        this.N = 624;
        this.M = 397;
        this.MATRIX_A = 0x9908b0df;
        this.UPPER_MASK = 0x80000000;
        this.LOWER_MASK = 0x7fffffff;
        this.mt = new Array(this.N);
        this.mti = this.N + 1;
        this.init_genrand(seed ?? new Date().getTime());
    }
    init_genrand(s) {
        this.mt[0] = s >>> 0;
        for (this.mti = 1; this.mti < this.N; this.mti++) {
            const prev = this.mt[this.mti - 1];
            const x = prev ^ (prev >>> 30);
            this.mt[this.mti] =
                (((((x & 0xffff0000) >>> 16) * 1812433253) << 16) +
                    (x & 0x0000ffff) * 1812433253 +
                    this.mti) >>>
                0;
        }
    }
    genrand_int32() {
        let y;
        const mag01 = [0, this.MATRIX_A];
        if (this.mti >= this.N) {
            let kk;
            for (kk = 0; kk < this.N - this.M; kk++) {
                y =
                    (this.mt[kk] & this.UPPER_MASK) |
                    (this.mt[kk + 1] & this.LOWER_MASK);
                this.mt[kk] = this.mt[kk + this.M] ^ (y >>> 1) ^ mag01[y & 1];
            }
            for (; kk < this.N - 1; kk++) {
                y =
                    (this.mt[kk] & this.UPPER_MASK) |
                    (this.mt[kk + 1] & this.LOWER_MASK);
                this.mt[kk] =
                    this.mt[kk + (this.M - this.N)] ^ (y >>> 1) ^ mag01[y & 1];
            }
            y =
                (this.mt[this.N - 1] & this.UPPER_MASK) |
                (this.mt[0] & this.LOWER_MASK);
            this.mt[this.N - 1] =
                this.mt[this.M - 1] ^ (y >>> 1) ^ mag01[y & 1];
            this.mti = 0;
        }
        y = this.mt[this.mti++];
        y ^= y >>> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >>> 18;
        return y >>> 0;
    }
    genrand_real2() {
        return this.genrand_int32() * (1.0 / 4294967296.0);
    }
}

class Random {
    constructor(seed) {
        this._mt = new MersenneTwister(seed);
        this._gaussNext = null;
    }
    seed(s) {
        this._mt.init_genrand(s);
    }
    getstate() {
        return { state: this._mt.mt.slice(), index: this._mt.mti };
    }
    setstate(st) {
        this._mt.mt = st.state.slice();
        this._mt.mti = st.index;
    }
    random() {
        return this._mt.genrand_real2();
    }
    getrandbits(k) {
        let result = 0n,
            bits = 0;
        while (bits < k) {
            const r = BigInt(this._mt.genrand_int32());
            const take = BigInt(Math.min(k - bits, 32));
            result |= (r & ((1n << take) - 1n)) << BigInt(bits);
            bits += Number(take);
        }
        return result;
    }
    randbytes(n) {
        // Uint8Array を使ってバイト列を生成し、ArrayBuffer を返す
        const ua = new Uint8Array(n);
        for (let i = 0; i < n; i++) {
            ua[i] = this._mt.genrand_int32() & 0xff;
        }
        return ua.buffer;
    }
    randrange(start, stop = null, step = 1) {
        if (stop === null) {
            stop = start;
            start = 0;
        }
        const width = stop - start;
        if (step === 1) return start + Math.floor(this.random() * width);
        const n = Math.floor((width + step - 1) / step);
        return start + step * Math.floor(this.random() * n);
    }
    randint(a, b) {
        return this.randrange(a, b + 1);
    }
    uniform(a, b) {
        return a + (b - a) * this.random();
    }
    triangular(low = 0, high = 1, mode = null) {
        if (mode === null) mode = (low + high) / 2;
        const u = this.random(),
            c = (mode - low) / (high - low);
        return u < c
            ? low + Math.sqrt(u * (high - low) * (mode - low))
            : high - Math.sqrt((1 - u) * (high - low) * (high - mode));
    }
    gauss(mu = 0, sigma = 1) {
        if (this._gaussNext !== null) {
            const v = this._gaussNext;
            this._gaussNext = null;
            return v * sigma + mu;
        }
        let u1, u2;
        do {
            u1 = this.random();
            u2 = this.random();
        } while (u1 <= Number.EPSILON);
        const mag = Math.sqrt(-2 * Math.log(u1));
        const z0 = mag * Math.cos(2 * Math.PI * u2),
            z1 = mag * Math.sin(2 * Math.PI * u2);
        this._gaussNext = z1;
        return z0 * sigma + mu;
    }
    normalvariate(mu, sigma) {
        return this.gauss(mu, sigma);
    }
    lognormvariate(mu, sigma) {
        return Math.exp(this.normalvariate(mu, sigma));
    }
    expovariate(lambd) {
        if (lambd <= 0) throw Error("lambda>0");
        return -Math.log(1 - this.random()) / lambd;
    }
    gammavariate(alpha, beta) {
        if (alpha > 1) {
            const d = alpha - 1 / 3,
                c = 1 / Math.sqrt(9 * d);
            while (true) {
                let x, v;
                do {
                    x = this.gauss();
                    v = 1 + c * x;
                } while (v <= 0);
                v = v * v * v;
                const u = this.random();
                if (u < 1 - 0.0331 * x * x * x * x) return d * v * beta;
                if (Math.log(u) < 0.5 * x * x + d * (1 - v + Math.log(v)))
                    return d * v * beta;
            }
        }
        if (alpha === 1) return -Math.log(1 - this.random()) * beta;
        return (
            this.gammavariate(alpha + 1, 1) *
            Math.pow(this.random(), 1 / alpha) *
            beta
        );
    }
    betavariate(a, b) {
        const y1 = this.gammavariate(a, 1),
            y2 = this.gammavariate(b, 1);
        return y1 / (y1 + y2);
    }
    paretovariate(a) {
        if (a <= 0) throw Error("alpha>0");
        return Math.pow(this.random(), -1 / a);
    }
    weibullvariate(a, b) {
        if (a <= 0 || b <= 0) throw Error("alpha/beta>0");
        return b * Math.pow(-Math.log(1 - this.random()), 1 / a);
    }
    vonmisesvariate(mu, kappa) {
        const TWO_PI = 2 * Math.PI;
        if (kappa <= 1e-6) return mu + TWO_PI * this.random();
        const a = 1 + Math.sqrt(1 + 4 * kappa * kappa),
            b = (a - Math.sqrt(2 * a)) / (2 * kappa),
            r = (1 + b * b) / (2 * b);
        while (true) {
            const u1 = this.random(),
                z = Math.cos(Math.PI * u1),
                f = (1 + r * z) / (r + z),
                c = kappa * (r - f),
                u2 = this.random();
            if (u2 < c * (2 - c) || u2 <= c * Math.exp(1 - c)) {
                const u3 = this.random(),
                    theta = u3 > 0.5 ? Math.acos(f) : -Math.acos(f);
                return (mu + theta + TWO_PI) % TWO_PI;
            }
        }
    }
    choice(seq) {
        if (seq.length === 0) throw Error("empty");
        return seq[Math.floor(this.random() * seq.length)];
    }
    choices(pop, weights = null, cum_weights = null, k = 1) {
        const n = pop.length;
        if (n === 0 || k < 0) throw Error("invalid");
        let cum = [];
        if (cum_weights) cum = cum_weights.slice();
        else if (weights) {
            let t = 0;
            for (let w of weights) {
                t += w;
                cum.push(t);
            }
        } else {
            for (let i = 0; i < n; i++) cum.push(i + 1);
        }
        const total = cum[cum.length - 1],
            res = [];
        for (let i = 0; i < k; i++) {
            const r = this.random() * total,
                idx = cum.findIndex((c) => r < c);
            res.push(pop[idx]);
        }
        return res;
    }
    shuffle(arr) {
        for (let i = arr.length - 1; i > 0; i--) {
            const j = Math.floor(this.random() * (i + 1));
            [arr[i], arr[j]] = [arr[j], arr[i]];
        }
        return arr;
    }
    sample(pop, k) {
        const n = pop.length;
        if (k < 0 || k > n) throw Error("invalid");
        const pool = pop.slice(),
            res = [];
        for (let i = 0; i < k; i++) {
            const idx = Math.floor(this.random() * pool.length);
            res.push(pool[idx]);
            pool.splice(idx, 1);
        }
        return res;
    }
}

export const __spec__ = {
    seed: { args: ["int"], ret: "void" },
    getstate: { args: [], ret: "str" },
    setstate: { args: ["str"], ret: "void" },
    random: { args: [], ret: "num" },
    getrandbits: { args: ["num"], ret: "num" },
    randbytes: { args: ["num"], ret: "str" },
    randrange: { args: ["num", "num", "num"], ret: "num" },
    randint: { args: ["num", "num"], ret: "num" },
    uniform: { args: ["num", "num"], ret: "num" },
    triangular: { args: ["num", "num", "num"], ret: "num" },
    gauss: { args: ["num", "num"], ret: "num" },
    normalvariate: { args: ["num", "num"], ret: "num" },
    lognormvariate: { args: ["num", "num"], ret: "num" },
    expovariate: { args: ["num"], ret: "num" },
    gammavariate: { args: ["num", "num"], ret: "num" },
    betavariate: { args: ["num", "num"], ret: "num" },
    paretovariate: { args: ["num"], ret: "num" },
    weibullvariate: { args: ["num", "num"], ret: "num" },
    vonmisesvariate: { args: ["num", "num"], ret: "num" },
    choice: { args: ["str"], ret: "str" },
    choices: { args: ["str", "str", "str", "num"], ret: "str" },
    shuffle: { args: ["str"], ret: "str" },
    sample: { args: ["str", "num"], ret: "str" },
};

export class MystiaRandomLib {
    constructor() {
        this.rng = new Random();
        this.functions = Object.create(null);
        for (const key of Object.keys(__spec__)) {
            this.functions[key] = (...ptrs) => {
                const args = ptrs.map((p, i) => {
                    const type = __spec__[key].args[i];
                    return read(this.instance, type, p);
                });
                const val = this.rng[key](...args);
                const retType = __spec__[key].ret;
                if (retType === "void") return;

                // ArrayBuffer / Uint8Array の判定を追加
                let payload;
                if (
                    val instanceof ArrayBuffer ||
                    ArrayBuffer.isView(val)
                ) {
                    // val が ArrayBuffer や TypedArray の場合、Uint8Array 化してから Base64 文字列に変換
                    const ua = val instanceof ArrayBuffer ? new Uint8Array(val) : new Uint8Array(val.buffer, val.byteOffset, val.byteLength);
                    payload = Buffer.from(ua).toString("base64");
                } else if (typeof val === "string" || typeof val === "number") {
                    payload = val;
                } else {
                    // その他の型はそのまま
                    payload = val;
                }

                // 返り値が文字列 or 数値なら write を呼び出す
                if (retType === "str" || retType === "num") {
                    return write(this.instance, retType, payload);
                } else {
                    return payload;
                }
            };
        }
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
