import { write, read } from "../ffi.mjs";

export const spec = {
    time: { args: [], ret: "num" },
    time_ns: { args: [], ret: "num" },
    perf_counter: { args: [], ret: "num" },
    perf_counter_ns: { args: [], ret: "num" },
    monotonic: { args: [], ret: "num" },
    monotonic_ns: { args: [], ret: "num" },
    process_time: { args: [], ret: "num" },
    process_time_ns: { args: [], ret: "num" },
    sleep: { args: ["num"], ret: "void" },
    ctime: { args: ["num"], ret: "str" },
    asctime: { args: ["tuple"], ret: "str" },
    localtime: { args: ["num"], ret: "tuple" },
    gmtime: { args: ["num"], ret: "tuple" },
    mktime: { args: ["tuple"], ret: "num" },
    strftime: { args: ["str", "tuple"], ret: "str" },
    strptime: { args: ["str", "str"], ret: "tuple" },
    tzset: { args: ["str"], ret: "void" },
    timezone: { args: [], ret: "num" },
    daylight: { args: [], ret: "bool" },
    tzname: { args: [], ret: "str" },
};

export class MystiaTimeLib {
    constructor() {
        this.functions = {
            time: () => write(this.instance, "num", Date.now() / 1000),
            time_ns: () => write(this.instance, "num", Date.now() * 1e6),
            perf_counter: () => {
                const [s, ns] = process.hrtime();
                return write(this.instance, "num", s + ns / 1e9);
            },
            perf_counter_ns: () => {
                const [s, ns] = process.hrtime();
                return write(this.instance, "num", s * 1e9 + ns);
            },
            monotonic: () => {
                const [s, ns] = process.hrtime();
                return write(this.instance, "num", s + ns / 1e9);
            },
            monotonic_ns: () => {
                const [s, ns] = process.hrtime();
                return write(this.instance, "num", s * 1e9 + ns);
            },
            process_time: () => {
                const u = process.cpuUsage();
                return write(this.instance, "num", (u.user + u.system) / 1e6);
            },
            process_time_ns: () => {
                const u = process.cpuUsage();
                return write(this.instance, "num", (u.user + u.system) * 1000);
            },
            sleep: (secs) => {
                const ms = read(this.instance, "num", secs) * 1000;
                const start = Date.now();
                while (Date.now() - start < ms) {
                    /* busy-wait */
                }
            },
            ctime: (secs) => {
                const t = read(this.instance, "num", secs);
                return write(
                    this.instance,
                    "str",
                    new Date((t || Date.now() / 1000) * 1000).toUTCString(),
                );
            },
            asctime: (tpl) => {
                const t = JSON.parse(read(this.instance, "str", tpl));
                // Python asctime: "Www Mmm dd hh:mm:ss yyyy"
                const d = new Date(
                    Date.UTC(t[0], t[1] - 1, t[2], t[3], t[4], t[5]),
                );
                return write(
                    this.instance,
                    "str",
                    d.toUTCString().replace(" GMT", ""),
                );
            },
            gmtime: (secs) => {
                const t = read(this.instance, "num", secs) || Date.now() / 1000;
                const d = new Date(t * 1000);
                const arr = [
                    d.getUTCFullYear(),
                    d.getUTCMonth() + 1,
                    d.getUTCDate(),
                    d.getUTCHours(),
                    d.getUTCMinutes(),
                    d.getUTCSeconds(),
                    d.getUTCDay(),
                    Math.floor(
                        (Date.UTC(
                            d.getUTCFullYear(),
                            d.getUTCMonth(),
                            d.getUTCDate(),
                        ) -
                            Date.UTC(d.getUTCFullYear(), 0, 1)) /
                            86400000,
                    ) + 1,
                    0,
                ];
                return write(this.instance, "str", JSON.stringify(arr));
            },
            localtime: (secs) => {
                const t = read(this.instance, "num", secs) || Date.now() / 1000;
                const d = new Date(t * 1000);
                const arr = [
                    d.getFullYear(),
                    d.getMonth() + 1,
                    d.getDate(),
                    d.getHours(),
                    d.getMinutes(),
                    d.getSeconds(),
                    d.getDay(),
                    Math.floor(
                        (d - new Date(d.getFullYear(), 0, 1)) / 86400000,
                    ) + 1,
                    d.getTimezoneOffset() < 0 ? 1 : 0,
                ];
                return write(this.instance, "str", JSON.stringify(arr));
            },
            mktime: (tpl) => {
                const t = JSON.parse(read(this.instance, "str", tpl));
                const d = new Date(t[0], t[1] - 1, t[2], t[3], t[4], t[5]);
                return write(this.instance, "num", d.getTime() / 1000);
            },
            strftime: (fmt, tpl) => {
                const f = read(this.instance, "str", fmt);
                const t = JSON.parse(read(this.instance, "str", tpl));
                const d = new Date(
                    Date.UTC(t[0], t[1] - 1, t[2], t[3], t[4], t[5]),
                );
                // minimal subset: %Y, %m, %d, %H, %M, %S
                const pad = (n) => n.toString().padStart(2, "0");
                return write(
                    this.instance,
                    "str",
                    f
                        .replace(/%Y/g, d.getUTCFullYear())
                        .replace(/%m/g, pad(d.getUTCMonth() + 1))
                        .replace(/%d/g, pad(d.getUTCDate()))
                        .replace(/%H/g, pad(d.getUTCHours()))
                        .replace(/%M/g, pad(d.getUTCMinutes()))
                        .replace(/%S/g, pad(d.getUTCSeconds())),
                );
            },
            strptime: (_s, _f) => {
                throw new Error("strptime not implemented");
            },
            tzset: (tz) => {
                process.env.TZ = read(this.instance, "str", tz);
            },
            timezone: () => {
                // seconds west of UTC
                return write(
                    this.instance,
                    "num",
                    -new Date().getTimezoneOffset() * 60,
                );
            },
            daylight: () => {
                // crude: assume DST if offset varies
                const jan = new Date(
                    new Date().getFullYear(),
                    0,
                    1,
                ).getTimezoneOffset();
                const jul = new Date(
                    new Date().getFullYear(),
                    6,
                    1,
                ).getTimezoneOffset();
                return jan !== jul;
            },
            tzname: () => {
                const fmt = new Intl.DateTimeFormat("en", {
                    timeZoneName: "short",
                });
                const parts = fmt.formatToParts(new Date());
                const name =
                    parts.find((p) => p.type === "timeZoneName")?.value || "";
                return write(this.instance, "str", name);
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
