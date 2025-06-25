import { write, read } from "../ffi.mjs";

export const spec = {
    now: { args: [], ret: "str" }, // current local datetime
    utcnow: { args: [], ret: "str" }, // current UTC datetime
    today: { args: [], ret: "str" }, // current local date
    date: { args: ["num", "num", "num"], ret: "str" }, // year, month, day
    time: { args: ["num", "num", "num", "num"], ret: "str" }, // hr, min, sec, μsec
    datetime: {
        args: ["num", "num", "num", "num", "num", "num", "num"],
        ret: "str",
    }, // y,m,d,hr,min,sec,μsec
    fromtimestamp: { args: ["num"], ret: "str" }, // seconds since epoch local
    utcfromtimestamp: { args: ["num"], ret: "str" }, // seconds since epoch UTC
    timestamp: { args: ["str"], ret: "num" }, // ISO string -> seconds since epoch
    strftime: { args: ["str", "str"], ret: "str" }, // dt_str, format
    strptime: { args: ["str", "str"], ret: "str" }, // text, format -> ISO
    isoformat: { args: ["str"], ret: "str" }, // ensure ISO formatting
    weekday: { args: ["str"], ret: "num" }, // 0=Mon..6=Sun
    isoweekday: { args: ["str"], ret: "num" }, // 1=Mon..7=Sun
    add_seconds: { args: ["str", "num"], ret: "str" }, // dt_str, seconds
    sub_seconds: { args: ["str", "num"], ret: "str" }, // dt_str, seconds
};

export class MystiaDatetimeLib {
    constructor() {
        this.functions = {
            now: () => write(
                this.instance,
                "str",
                new Date().toString(),
            ),
            utcnow: () => write(
                this.instance,
                "str",
                new Date().toUTCString(),
            ),
            today: () => {
                let date = new Date()
                    .toISOString()
                    .slice(0, 10)
                    .split("-")
                    .map((x) => parseInt(x));
                return write(
                    this.instance,
                    { type: "array", element: "int", length: 3 },
                    date,
                );
            },
            date: (y, y_typ, m, m_typ, d, d_typ) => {
                const Y = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", y_typ).toString()})`,
                    ),
                    y,
                );
                const M = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", m_typ).toString()})`,
                    ),
                    m,
                );
                const D = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", d_typ).toString()})`,
                    ),
                    d,
                );
                const dt = new Date(Y, M - 1, D);
                return write(
                    this.instance,
                    "str",
                    dt.toISOString().slice(0, 10),
                );
            },
            time: (hr, hr_typ, min, min_typ, sec, sec_typ, μ, μ_typ) => {
                const pad = (n) => String(n).padStart(2, "0");
                const H = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", hr_typ).toString()})`,
                    ),
                    hr,
                );
                const MI = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", min_typ).toString()})`,
                    ),
                    min,
                );
                const S = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", sec_typ).toString()})`,
                    ),
                    sec,
                );
                const MS = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", μ_typ).toString()})`,
                    ),
                    μ,
                );
                return write(
                    this.instance,
                    "str",
                    `${pad(H)}:${pad(MI)}:${pad(S)}.${String(MS).padStart(6, "0")}`,
                );
            },
            datetime: (y, y_typ, m, m_typ, d, d_typ, hr, hr_typ, min, min_typ, sec, sec_typ, μ, μ_typ) => {
                const Y = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", y_typ).toString()})`,
                    ),
                    y,
                );
                const M = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", m_typ).toString()})`,
                    ),
                    m,
                );
                const D = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", d_typ).toString()})`,
                    ),
                    d,
                );
                const H = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", hr_typ).toString()})`,
                    ),
                    hr,
                );
                const MI = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", min_typ).toString()})`,
                    ),
                    min,
                );
                const S = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", sec_typ).toString()})`,
                    ),
                    sec,
                );
                const MS = Math.floor(
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", μ_typ).toString()})`,
                        ),
                        μ,
                    ) / 1000,
                );
                const dt = new Date(Y, M - 1, D, H, MI, S, MS);
                return write(
                    this.instance,
                    "str",
                    dt.toISOString(),
                );
            },
            fromtimestamp: (sec, sec_typ) => {
                const S = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", sec_typ).toString()})`,
                    ),
                    sec,
                );
                return write(
                    this.instance,
                    "str",
                    new Date(S * 1000).toString(),
                );
            },
            utcfromtimestamp: (sec, sec_typ) => {
                const S = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", sec_typ).toString()})`,
                    ),
                    sec,
                );
                return write(
                    this.instance,
                    "str",
                    new Date(S * 1000).toUTCString(),
                );
            },
            timestamp: (iso, iso_typ) => {
                const I = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", iso_typ).toString()})`,
                    ),
                    iso,
                );
                return Date.parse(I) / 1000;
            },
            strftime: (iso, iso_typ, fmt, fmt_typ) => {
                const I = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", iso_typ).toString()})`,
                    ),
                    iso,
                );
                const F = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", fmt_typ).toString()})`,
                    ),
                    fmt,
                );
                const dt = new Date(I);
                return write(
                    this.instance,
                    "str",
                    new Intl.DateTimeFormat("en-US", {
                        dateStyle: "short",
                        timeStyle: "medium",
                    }).format(dt),
                );
            },
            strptime: (text, text_typ, fmt, fmt_typ) => {
                const T = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", text_typ).toString()})`,
                    ),
                    text,
                );
                const dt = new Date(T);
                return write(
                    this.instance,
                    "str",
                    dt.toISOString(),
                );
            },
            isoformat: (iso, iso_typ) => {
                const I = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", iso_typ).toString()})`,
                    ),
                    iso,
                );
                return write(
                    this.instance,
                    "str",
                    new Date(I).toISOString(),
                );
            },
            weekday: (iso, iso_typ) => {
                const I = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", iso_typ).toString()})`,
                    ),
                    iso,
                );
                const d = new Date(I).getDay();
                return d === 0 ? 6 : d - 1;
            },
            isoweekday: (iso, iso_typ) => {
                const I = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", iso_typ).toString()})`,
                    ),
                    iso,
                );
                const d = new Date(I).getDay();
                return d === 0 ? 7 : d;
            },
            add_seconds: (iso, iso_typ, sec, sec_typ) => {
                const I = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", iso_typ).toString()})`,
                    ),
                    iso,
                );
                const S = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", sec_typ).toString()})`,
                    ),
                    sec,
                );
                return write(
                    this.instance,
                    "str",
                    new Date(
                        Date.parse(I) + S * 1000,
                    ).toISOString(),
                );
            },
            sub_seconds: (iso, iso_typ, sec, sec_typ) => {
                const I = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", iso_typ).toString()})`,
                    ),
                    iso,
                );
                const S = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", sec_typ).toString()})`,
                    ),
                    sec,
                );
                return write(
                    this.instance,
                    "str",
                    new Date(
                        Date.parse(I) - S * 1000,
                    ).toISOString(),
                );
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