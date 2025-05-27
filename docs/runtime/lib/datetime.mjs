import { write, read } from "../ffi.mjs";

export const spec = {
  now:           { args: [],                                      ret: "str" },  // current local datetime
  utcnow:        { args: [],                                      ret: "str" },  // current UTC datetime
  today:         { args: [],                                      ret: "str" },  // current local date
  date:          { args: ["num","num","num"],                ret: "str" },  // year, month, day
  time:          { args: ["num","num","num","num"],        ret: "str" },  // hr, min, sec, μsec
  datetime:      { args: ["num","num","num","num","num","num","num"], ret: "str" },  // y,m,d,hr,min,sec,μsec
  fromtimestamp: { args: ["num"],                                ret: "str" },  // seconds since epoch local
  utcfromtimestamp: { args: ["num"],                             ret: "str" },  // seconds since epoch UTC
  timestamp:     { args: ["str"],                                ret: "num" },  // ISO string -> seconds since epoch
  strftime:      { args: ["str","str"],                        ret: "str" },  // dt_str, format
  strptime:      { args: ["str","str"],                        ret: "str" },  // text, format -> ISO
  isoformat:     { args: ["str"],                                ret: "str" },  // ensure ISO formatting
  weekday:       { args: ["str"],                                ret: "num" },  // 0=Mon..6=Sun
  isoweekday:    { args: ["str"],                                ret: "num" },  // 1=Mon..7=Sun
  add_seconds:   { args: ["str","num"],                        ret: "str" },  // dt_str, seconds
  sub_seconds:   { args: ["str","num"],                        ret: "str" },  // dt_str, seconds
};

export class MystiaDatetimeLib {
  constructor() {
    this.functions = {
      now: () => write(this.instance, "str", new Date().toString()),
      utcnow: () => write(this.instance, "str", new Date().toUTCString()),
      today: () => write(this.instance, "str", new Date().toISOString().slice(0,10)),
      date: (y,m,d) => {
        const dt = new Date(read(this.instance,"num",y), read(this.instance,"num",m)-1, read(this.instance,"num",d));
        return write(this.instance, "str", dt.toISOString().slice(0,10));
      },
      time: (hr,min,sec,μ) => {
        const pad = n => String(n).padStart(2,'0');
        return write(this.instance, "str", `${pad(read(this.instance,"num",hr))}:${pad(read(this.instance,"num",min))}:${pad(read(this.instance,"num",sec))}.${String(read(this.instance,"num",μ)).padStart(6,'0')}`);
      },
      datetime: (y,m,d,hr,min,sec,μ) => {
        const dt = new Date(
          read(this.instance,"num",y),
          read(this.instance,"num",m)-1,
          read(this.instance,"num",d),
          read(this.instance,"num",hr),
          read(this.instance,"num",min),
          read(this.instance,"num",sec),
          Math.floor(read(this.instance,"num",μ)/1000)
        );
        return write(this.instance, "str", dt.toISOString());
      },
      fromtimestamp: (sec) => write(this.instance, "str", new Date(read(this.instance,"num",sec)*1000).toString()),
      utcfromtimestamp: (sec) => write(this.instance, "str", new Date(read(this.instance,"num",sec)*1000).toUTCString()),
      timestamp: (iso) => Date.parse(read(this.instance,"str",iso)) / 1000,
      strftime: (iso,fmt) => {
        const dt = new Date(read(this.instance,"str",iso));
        return write(this.instance, "str", new Intl.DateTimeFormat('en-US', { dateStyle: 'short', timeStyle: 'medium' }).format(dt));
      },
      strptime: (text, fmt) => {
        const dt = new Date(read(this.instance,"str",text));
        return write(this.instance, "str", dt.toISOString());
      },
      isoformat: (iso) => write(this.instance, "str", new Date(read(this.instance,"str",iso)).toISOString()),
      weekday: (iso) => new Date(read(this.instance,"str",iso)).getDay() === 0 ? 6 : new Date(read(this.instance,"str",iso)).getDay() - 1,
      isoweekday: (iso) => new Date(read(this.instance,"str",iso)).getDay() === 0 ? 7 : new Date(read(this.instance,"str",iso)).getDay(),
      add_seconds: (iso,sec) => write(this.instance, "str", new Date(Date.parse(read(this.instance,"str",iso)) + read(this.instance,"num",sec)*1000).toISOString()),
      sub_seconds: (iso,sec) => write(this.instance, "str", new Date(Date.parse(read(this.instance,"str",iso)) - read(this.instance,"num",sec)*1000).toISOString()),
    };
  }

  set_wasm(instance) {
    this.instance = instance;
  }

  bridge() {
    return Object.fromEntries(
      Object.entries(this.functions).map(([k,v]) => [k, (...a) => v(...a)])
    );
  }
}
