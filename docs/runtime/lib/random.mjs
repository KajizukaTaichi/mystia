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
    seed(s, s_typ) {
        this._mt.init_genrand(
            read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", s_typ).toString()})`,
                ),
                s,
            ),
        );
    }
    getstate() {
        const st = { state: this._mt.mt.slice(), index: this._mt.mti };
        return write(this.instance, "str", JSON.stringify(st));
    }
    setstate(st, st_typ) {
        const obj = JSON.parse(
            read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", st_typ).toString()})`,
                ),
                st,
            ),
        );
        this._mt.mt = obj.state.slice();
        this._mt.mti = obj.index;
    }
    random() {
        return write(this.instance, "num", this._mt.genrand_real2());
    }
    getrandbits(k, k_typ) {
        let result = 0n,
            bits = 0;
        const need = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", k_typ).toString()})`,
            ),
            k,
        );
        while (bits < need) {
            const r = BigInt(this._mt.genrand_int32());
            const take = BigInt(Math.min(need - bits, 32));
            result |= (r & ((1n << take) - 1n)) << BigInt(bits);
            bits += Number(take);
        }
        return write(this.instance, "num", result);
    }
    randbytes(n, n_typ) {
        const count = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", n_typ).toString()})`,
            ),
            n,
        );
        const ua = new Uint8Array(count);
        for (let i = 0; i < count; i++) {
            ua[i] = this._mt.genrand_int32() & 0xff;
        }
        const payload = Buffer.from(ua).toString("base64");
        return write(this.instance, "str", payload);
    }
    randrange(start, start_typ, stop = null, stop_typ, step = 1, step_typ) {
        let s = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", start_typ).toString()})`,
            ),
            start,
        );
        let e;
        if (stop === null) {
            e = s;
            s = 0;
        } else {
            e = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", stop_typ).toString()})`,
                ),
                stop,
            );
        }
        const stp = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", step_typ).toString()})`,
            ),
            step,
        );
        const width = e - s;
        if (stp === 1) {
            return write(
                this.instance,
                "num",
                s + Math.floor(this.random() * width),
            );
        }
        const nOptions = Math.floor((width + stp - 1) / stp);
        return write(
            this.instance,
            "num",
            s + stp * Math.floor(this.random() * nOptions),
        );
    }
    randint(a, a_typ, b, b_typ) {
        const start = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", a_typ).toString()})`,
            ),
            a,
        );
        const end = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", b_typ).toString()})`,
            ),
            b,
        );
        return this.randrange(start, null, end + 1, null, 1, null);
    }
    uniform(a, a_typ, b, b_typ) {
        const low = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", a_typ).toString()})`,
            ),
            a,
        );
        const high = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", b_typ).toString()})`,
            ),
            b,
        );
        return write(
            this.instance,
            "num",
            low + (high - low) * this.random(),
        );
    }
    triangular(low, low_typ, high, high_typ, mode = null, mode_typ) {
        const lo = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", low_typ).toString()})`,
            ),
            low,
        );
        const hi = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", high_typ).toString()})`,
            ),
            high,
        );
        let md;
        if (mode === null) {
            md = (lo + hi) / 2;
        } else {
            md = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", mode_typ).toString()})`,
                ),
                mode,
            );
        }
        const u = this.random();
        const c = (md - lo) / (hi - lo);
        const result =
            u < c
                ? lo + Math.sqrt(u * (hi - lo) * (md - lo))
                : hi - Math.sqrt((1 - u) * (hi - lo) * (hi - md));
        return write(this.instance, "num", result);
    }
    gauss(mu, mu_typ, sigma, sigma_typ) {
        if (this._gaussNext !== null) {
            const v = this._gaussNext;
            this._gaussNext = null;
            return write(
                this.instance,
                "num",
                v *
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", sigma_typ).toString()})`,
                        ),
                        sigma,
                    ) +
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", mu_typ).toString()})`,
                        ),
                        mu,
                    ),
            );
        }
        let u1, u2;
        do {
            u1 = this.random();
            u2 = this.random();
        } while (u1 <= Number.EPSILON);
        const mag = Math.sqrt(-2 * Math.log(u1));
        const z0 = mag * Math.cos(2 * Math.PI * u2);
        const z1 = mag * Math.sin(2 * Math.PI * u2);
        this._gaussNext = z1;
        return write(
            this.instance,
            "num",
            z0 *
                read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", sigma_typ).toString()})`,
                    ),
                    sigma,
                ) +
                read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", mu_typ).toString()})`,
                    ),
                    mu,
                ),
        );
    }
    normalvariate(mu, mu_typ, sigma, sigma_typ) {
        const m = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", mu_typ).toString()})`,
            ),
            mu,
        );
        const s = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", sigma_typ).toString()})`,
            ),
            sigma,
        );
        return this.gauss(m, null, s, null);
    }
    lognormvariate(mu, mu_typ, sigma, sigma_typ) {
        const val = this.normalvariate(mu, mu_typ, sigma, sigma_typ);
        return write(
            this.instance,
            "num",
            Math.exp(val),
        );
    }
    expovariate(lambd, lambd_typ) {
        const l = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", lambd_typ).toString()})`,
            ),
            lambd,
        );
        if (l <= 0) throw Error("lambda>0");
        return write(
            this.instance,
            "num",
            -Math.log(1 - this.random()) / l,
        );
    }
    gammavariate(alpha, alpha_typ, beta, beta_typ) {
        const a = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", alpha_typ).toString()})`,
            ),
            alpha,
        );
        const b = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", beta_typ).toString()})`,
            ),
            beta,
        );
        if (a > 1) {
            const d = a - 1 / 3,
                cVal = 1 / Math.sqrt(9 * d);
            while (true) {
                let x, v;
                do {
                    x = this.gauss(0, null, 1, null);
                    v = 1 + cVal * x;
                } while (v <= 0);
                v = v * v * v;
                const u = this.random();
                if (u < 1 - 0.0331 * x * x * x * x) return write(this.instance, "num", d * v * b);
                if (Math.log(u) < 0.5 * x * x + d * (1 - v + Math.log(v))) return write(this.instance, "num", d * v * b);
            }
        }
        if (a === 1) {
            const res = -Math.log(1 - this.random()) * b;
            return write(this.instance, "num", res);
        }
        const val = this.gammavariate(a + 1, null, 1, null) * Math.pow(this.random(), 1 / a) * b;
        return write(this.instance, "num", val);
    }
    betavariate(a, a_typ, b, b_typ) {
        const y1 = this.gammavariate(a, a_typ, 1, null);
        const y2 = this.gammavariate(b, b_typ, 1, null);
        return write(this.instance, "num", y1 / (y1 + y2));
    }
    paretovariate(a, a_typ) {
        const x = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", a_typ).toString()})`,
            ),
            a,
        );
        if (x <= 0) throw Error("alpha>0");
        return write(this.instance, "num", Math.pow(this.random(), -1 / x));
    }
    weibullvariate(a, a_typ, b, b_typ) {
        const av = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", a_typ).toString()})`,
            ),
            a,
        );
        const bv = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", b_typ).toString()})`,
            ),
            b,
        );
        if (av <= 0 || bv <= 0) throw Error("alpha/beta>0");
        return write(this.instance, "num", bv * Math.pow(-Math.log(1 - this.random()), 1 / av));
    }
    vonmisesvariate(mu, mu_typ, kappa, kappa_typ) {
        const mVal = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", mu_typ).toString()})`,
            ),
            mu,
        );
        const kVal = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", kappa_typ).toString()})`,
            ),
            kappa,
        );
        const TWO_PI = 2 * Math.PI;
        if (kVal <= 1e-6) return write(this.instance, "num", mVal + TWO_PI * this.random());
        const a = 1 + Math.sqrt(1 + 4 * kVal * kVal);
        const bVal = (a - Math.sqrt(2 * a)) / (2 * kVal);
        const r = (1 + bVal * bVal) / (2 * bVal);
        while (true) {
            const u1 = this.random();
            const z = Math.cos(Math.PI * u1);
            const f = (1 + r * z) / (r + z);
            const c = kVal * (r - f);
            const u2 = this.random();
            if (u2 < c * (2 - c) || u2 <= c * Math.exp(1 - c)) {
                const u3 = this.random();
                const theta = u3 > 0.5 ? Math.acos(f) : -Math.acos(f);
                return write(this.instance, "num", (mVal + theta + TWO_PI) % TWO_PI);
            }
        }
    }
    choice(seq, seq_typ) {
        const arr = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", seq_typ).toString()})`,
            ),
            seq,
        );
        if (arr.length === 0) throw Error("empty");
        const idx = Math.floor(this.random() * arr.length);
        return write(this.instance, "str", arr[idx]);
    }
    choices(pop, pop_typ, weights = null, weights_typ, cum_weights = null, cum_typ, k = 1, k_typ) {
        const p = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", pop_typ).toString()})`,
            ),
            pop,
        );
        let wArr = null;
        let cArr = null;
        if (weights !== null) {
            wArr = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", weights_typ).toString()})`,
                ),
                weights,
            );
        }
        if (cum_weights !== null) {
            cArr = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", cum_typ).toString()})`,
                ),
                cum_weights,
            );
        }
        const count = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", k_typ).toString()})`,
            ),
            k,
        );
        const n = p.length;
        if (n === 0 || count < 0) throw Error("invalid");
        let cum = [];
        if (cArr) cum = cArr.slice();
        else if (wArr) {
            let t = 0;
            for (let w of wArr) {
                t += w;
                cum.push(t);
            }
        } else {
            for (let i = 0; i < n; i++) cum.push(i + 1);
        }
        const total = cum[cum.length - 1];
        const res = [];
        for (let i = 0; i < count; i++) {
            const r = this.random() * total;
            const idx = cum.findIndex((c) => r < c);
            res.push(p[idx]);
        }
        return write(this.instance, "str", JSON.stringify(res));
    }
    shuffle(arr, arr_typ) {
        const a = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", arr_typ).toString()})`,
            ),
            arr,
        );
        for (let i = a.length - 1; i > 0; i--) {
            const j = Math.floor(this.random() * (i + 1));
            [a[i], a[j]] = [a[j], a[i]];
        }
        return write(this.instance, "str", JSON.stringify(a));
    }
    sample(pop, pop_typ, k, k_typ) {
        const p = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", pop_typ).toString()})`,
            ),
            pop,
        );
        const count = read(
            this.instance,
            eval(
                `(${read(this.instance, "str", k_typ).toString()})`,
            ),
            k,
        );
        const n = p.length;
        if (count < 0 || count > n) throw Error("invalid");
        const pool = p.slice();
        const res = [];
        for (let i = 0; i < count; i++) {
            const idx = Math.floor(this.random() * pool.length);
            res.push(pool[idx]);
            pool.splice(idx, 1);
        }
        return write(this.instance, "str", JSON.stringify(res));
    }
}

export const __spec__ = {
    seed: { args: ["int"], ret: "void" },
    getstate: { args: [], ret: "str" },
    setstate: { args: ["str"], ret: "void" },
    random: { args: [], ret: "num" },
    getrandbits: { args: ["num"], ret: "num" },
    randbytes: { args: ["num"], ret: "str" },
    randrange: { args: ["num", "num", "num", "num", "num", "num"], ret: "num" },
    randint: { args: ["num", "num", "num", "num", "num", "num"], ret: "num" },
    uniform: { args: ["num", "num", "num", "num"], ret: "num" },
    triangular: { args: ["num", "num", "num", "num", "num", "num", "num", "num", "num", "num", "num", "num"], ret: "num" },
    gauss: { args: ["num", "num", "num", "num"], ret: "num" },
    normalvariate: { args: ["num", "num", "num", "num"], ret: "num" },
    lognormvariate: { args: ["num", "num", "num", "num"], ret: "num" },
    expovariate: { args: ["num", "num"], ret: "num" },
    gammavariate: { args: ["num", "num", "num", "num"], ret: "num" },
    betavariate: { args: ["num", "num", "num", "num"], ret: "num" },
    paretovariate: { args: ["num", "num"], ret: "num" },
    weibullvariate: { args: ["num", "num", "num", "num"], ret: "num" },
    vonmisesvariate: { args: ["num", "num", "num", "num"], ret: "num" },
    choice: { args: ["str", "str"], ret: "str" },
    choices: { args: ["str", "str", "str", "str", "str", "str", "num", "num"], ret: "str" },
    shuffle: { args: ["str", "str"], ret: "str" },
    sample: { args: ["str", "str", "num", "num"], ret: "str" },
};

export class MystiaRandomLib {
    constructor() {
        this.rng = new Random();
        this.functions = Object.create(null);
        for (const key of Object.keys(__spec__)) {
            this.functions[key] = (...ptrs) => {
                const args = [];
                for (let i = 0; i < ptrs.length; i += 2) {
                    const ptr = ptrs[i];
                    const typPtr = ptrs[i + 1];
                    args.push(
                        read(
                            this.instance,
                            eval(
                                `(${read(
                                    this.instance,
                                    "str",
                                    typPtr,
                                ).toString()})`,
                            ),
                            ptr,
                        ),
                    );
                }
                const val = this.rng[key](...args);
                const retType = __spec__[key].ret;
                if (retType === "void") return;
                let payload;
                if (
                    val instanceof ArrayBuffer ||
                    ArrayBuffer.isView(val)
                ) {
                    const ua =
                        val instanceof ArrayBuffer
                            ? new Uint8Array(val)
                            : new Uint8Array(
                                  val.buffer,
                                  val.byteOffset,
                                  val.byteLength,
                              );
                    payload = Buffer.from(ua).toString("base64");
                } else {
                    payload = val;
                }
                if (retType === "str" || retType === "num") {
                    return write(
                        this.instance,
                        retType,
                        payload,
                    );
                }
                return payload;
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