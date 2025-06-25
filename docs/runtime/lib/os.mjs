import { write, read } from "../ffi.mjs";
import fs from "fs";
import path from "path";

export class MystiaOSLib {
    constructor() {
        this.functions = {
            getcwd: () => {
                return write(
                    this.instance,
                    "str",
                    process.cwd(),
                );
            },
            remove: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return fs.unlinkSync(V);
            },
            mkdir: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return fs.mkdirSync(V);
            },
            rename: (src, src_typ, dest, dest_typ) => {
                const S = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", src_typ).toString()})`,
                    ),
                    src,
                );
                const D = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", dest_typ).toString()})`,
                    ),
                    dest,
                );
                return fs.renameSync(S, D);
            },
            chdir: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return write(
                    this.instance,
                    "str",
                    process.chdir(V),
                );
            },
            listdir: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                const L = fs.readdirSync(V);
                return write(
                    this.instance,
                    "str",
                    L,
                );
            },
            path_join: (value, value_typ, end, end_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                const E = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", end_typ).toString()})`,
                    ),
                    end,
                );
                return write(
                    this.instance,
                    "str",
                    path.join(V, E),
                );
            },
            path_basename: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return write(
                    this.instance,
                    "str",
                    path.basename(V),
                );
            },
            path_parent: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return write(
                    this.instance,
                    "str",
                    path.dirname(V),
                );
            },
            path_abs: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return write(
                    this.instance,
                    "str",
                    path.resolve(V),
                );
            },
            path_exist: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return fs.existsSync(V);
            },
            path_isfile: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                let isFile = false;
                try {
                    isFile = fs
                        .statSync(V)
                        .isFile();
                } catch {
                    isFile = false;
                }
                return isFile;
            },
            path_isdir: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                let isDir = false;
                try {
                    isDir = fs
                        .statSync(V)
                        .isDirectory();
                } catch {
                    isDir = false;
                }
                return isDir;
            },
            path_isabs: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return path.isAbsolute(V);
            },
            path_root: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return write(
                    this.instance,
                    "str",
                    path.parse(V).root,
                );
            },
            path_ext: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return write(
                    this.instance,
                    "str",
                    path.extname(V),
                );
            },
            read_file: (value, value_typ) => {
                const V = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", value_typ).toString()})`,
                    ),
                    value,
                );
                return write(
                    this.instance,
                    "str",
                    fs.readFileSync(V, "utf8"),
                );
            },
            write_file: (file, file_typ, content, content_typ) => {
                const F = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", file_typ).toString()})`,
                    ),
                    file,
                );
                const C = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", content_typ).toString()})`,
                    ),
                    content,
                );
                fs.writeFileSync(
                    F,
                    C,
                    "utf8",
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
